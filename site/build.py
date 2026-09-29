#!/usr/bin/env python3
"""Build the lessons website into docs/ for GitHub Pages.

Each lesson's header comment becomes the page intro, each `// Example N:` block becomes
a section with its highlighted code and the output it printed, and the whole file is
shown at the end with a Rust Playground button.

Usage: python3 site/build.py            # builds the lessons, runs them, writes docs/
       python3 -m http.server -d docs   # preview at http://localhost:8000
"""

import html
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SITE = ROOT / "site"
OUT = ROOT / "docs"
REPO = "https://github.com/Clivern/Buzzard"

PARTS = [
    ("Basics", 1, 9),
    ("Ownership and borrowing", 10, 13),
    ("Structs, enums, and errors", 14, 19),
    ("Standard traits", 20, 24),
    ("Generics and traits", 25, 29),
    ("Iterators, closures, and lifetimes", 30, 35),
    ("Collections", 36, 39),
    ("Smart pointers and concurrency", 40, 46),
    ("The standard library", 47, 52),
    ("Organizing code", 53, 56),
    ("Advanced", 57, 60),
]

# Lessons the playground cannot run as a single file.
NO_PLAYGROUND = {"53_modules"}

EXAMPLE = re.compile(r"^// Example (\d+): (.*?)\.?$")
TOP_FN = re.compile(r"^(pub )?(async )?(unsafe )?fn ")
SECTION = re.compile(r"^-- .* --$")


# ---------------------------------------------------------------------------
# Rust highlighting

KEYWORDS = set(
    "as async await break const continue crate dyn else enum extern false fn for if "
    "impl in let loop match mod move mut pub ref return self Self static struct super "
    "trait true type union unsafe use where while yield".split()
)

PRIMITIVES = set(
    "i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char str".split()
)

TOKEN = re.compile(
    r"""
    (?P<comment>//[^\n]*)
    |(?P<block>/\*[\s\S]*?\*/)
    |(?P<rawstr>\b[bc]?r(?P<hashes>\#*)"[\s\S]*?"(?P=hashes))
    |(?P<str>\b[bc]?"(?:\\[\s\S]|[^"\\])*"|"(?:\\[\s\S]|[^"\\])*")
    |(?P<char>b?'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^'\\\n])')
    |(?P<life>'[A-Za-z_]\w*)
    |(?P<attr>\#!?\[[^\]\n]*\])
    |(?P<num>\b(?:0x[0-9a-fA-F_]+|0b[01_]+|0o[0-7_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?)
        (?:_?(?:[iu](?:8|16|32|64|128|size)|f32|f64))?)
    |(?P<macro>\b[A-Za-z_]\w*!(?=[\s(\[{]))
    |(?P<ident>\b[A-Za-z_]\w*\b)
    """,
    re.VERBOSE,
)

ERROR_COMMENT = re.compile(r"//\s*(error|warning)(\[\w+\])?:")


def highlight(code):
    out = []
    pos = 0
    prev = None
    for m in TOKEN.finditer(code):
        out.append(html.escape(code[pos : m.start()]))
        text = m.group()
        kind = m.lastgroup
        cls = None
        if kind == "comment":
            cls = "e" if ERROR_COMMENT.match(text) else "c"
        elif kind == "block":
            cls = "c"
        elif kind in ("rawstr", "str", "char"):
            cls = "s"
        elif kind in ("life", "attr"):
            cls = "a"
        elif kind == "num":
            cls = "n"
        elif kind == "macro":
            cls = "m"
        elif kind == "ident":
            if text in KEYWORDS:
                cls = "k"
            elif text in PRIMITIVES or text[0].isupper():
                cls = "t"
            elif prev == "fn":
                cls = "f"

            prev = text

        esc = html.escape(text)
        out.append(f'<span class="{cls}">{esc}</span>' if cls else esc)
        pos = m.end()

    out.append(html.escape(code[pos:]))
    return "".join(out)


# ---------------------------------------------------------------------------
# Header prose

KNOWN_TYPES = set(
    "Vec Cell Rc Arc Box Option Result Some None Ok Err String Mutex Send Sync Copy "
    "Clone Drop Deref Display Debug Default Iterator Fn FnMut FnOnce Sized Self Ord Eq "
    "Hash From Into Add Pin Future Waker Poll Cow Path Duration Instant".split()
)
LETTERS = set("TUEKVR")
LEAD = "(\"“"
TRAIL = ".,;:\"”"


def looks_like_code(word):
    if word in KNOWN_TYPES or word in PRIMITIVES or word in LETTERS:
        return True

    if word in ("_", "@", "?", "..", "!", "*", "&"):
        return True

    if re.search(r"::|!$|!\(|\w\(|[<>{}\[\]#&*]|^'[a-z]|\w_\w|^_\w|^--\w|\w\.rs$", word):
        return True

    # CamelCase such as RefCell or JoinHandle.
    return bool(re.fullmatch(r"[A-Z][a-z0-9]+(?:[A-Z][a-z0-9]*)+", word))


def capitalize(label):
    """Capitalize a heading unless it starts with code, such as `cfg` or `impl`."""
    first = label.split()[0]
    if first in KEYWORDS or first == "cfg" or looks_like_code(first.rstrip(",:")):
        return label

    return label[:1].upper() + label[1:]


def split_punct(word):
    lead = ""
    while word and word[0] in LEAD:
        lead += word[0]
        word = word[1:]

    trail = ""
    while len(word) > 1 and (
        (word[-1] in TRAIL and not word.endswith("::"))
        or (word[-1] == ")" and word.count("(") < word.count(")"))
    ):
        trail = word[-1] + trail
        word = word[:-1]

    return lead, word, trail


def inline(text):
    """Escape prose, wrapping code-looking words (and runs of them) in <code>."""
    # Each piece is (html, glued): glued pieces attach to the previous one without a space.
    pieces = []
    run = []
    run_glued = False

    def flush():
        if run:
            pieces.append(("<code>" + html.escape(" ".join(run)) + "</code>", run_glued))
            run.clear()

    for word in text.split():
        lead, core, trail = split_punct(word)
        if core and looks_like_code(core):
            if lead:
                flush()
                pieces.append((html.escape(lead), False))

            if not run:
                run_glued = bool(lead)

            run.append(core)
            if trail:
                flush()
                pieces.append((html.escape(trail), True))
        else:
            flush()
            pieces.append((html.escape(word), False))

    flush()
    out = ""
    for piece, glued in pieces:
        out += piece if glued or not out else " " + piece

    return out


def prose(lines):
    """Turn header comment lines into <p> and <ul> blocks."""
    blocks = []
    para = []
    items = []

    def end_para():
        if para:
            blocks.append("<p>" + inline(" ".join(para)) + "</p>")
            para.clear()

    def end_list():
        if items:
            lis = "".join(f"<li>{inline(i)}</li>" for i in items)
            blocks.append(f"<ul>{lis}</ul>")
            items.clear()

    for line in lines:
        if not line.strip():
            end_para()
            end_list()
        elif line.startswith("- "):
            end_para()
            items.append(line[2:])
        elif line.startswith("  ") and items:
            items[-1] += " " + line.strip()
        else:
            end_list()
            para.append(line)

    end_para()
    end_list()
    return "\n".join(blocks)


# ---------------------------------------------------------------------------
# Lesson parsing


def parse(folder):
    text = (folder / "main.rs").read_text()
    lines = text.splitlines()

    # Skip crate docs (//!) above the lesson header.
    i = 0
    while lines[i].startswith("//!") or not lines[i].strip():
        i += 1

    comment = []
    while i < len(lines) and lines[i].startswith("//"):
        comment.append(lines[i][3:] if lines[i].startswith("// ") else lines[i][2:])
        i += 1

    num, title = re.match(r"(\d+)\. (.*)", comment[0]).groups()
    command = re.compile(r"^(Run|Test): (cargo .*)")
    commands = [command.match(l).group(2) for l in comment if command.match(l)]
    body = [l for l in comment[1:] if not command.match(l)]

    examples = []
    cursor = i
    for m in [j for j in range(i, len(lines)) if EXAMPLE.match(lines[j])]:
        n, label = EXAMPLE.match(lines[m]).groups()
        fn = next(j for j in range(m + 1, len(lines)) if TOP_FN.match(lines[j]))
        end = next(j for j in range(fn, len(lines)) if lines[j] == "}")

        # A section runs from the end of the previous example's function, so helpers
        # and statics declared just before an example stay with it.
        chunk = [l for k, l in enumerate(lines[cursor : end + 1]) if cursor + k != m]
        while chunk and (not chunk[0].strip() or chunk[0].startswith("use ")):
            chunk.pop(0)

        code = "\n".join(chunk).rstrip()
        examples.append(
            {
                "n": int(n),
                "label": capitalize(label),
                "code": code,
                "headers": re.findall(r'"(-- [^"{}]*? --)"', code),
            }
        )
        cursor = end + 1

    extras = []
    for p in sorted(folder.rglob("*")):
        if p.is_file() and p.name != "main.rs":
            extras.append((p.relative_to(folder).as_posix(), p.read_text().rstrip()))

    return {
        "slug": folder.name,
        "num": int(num),
        "title": title,
        "intro": prose(body),
        "commands": commands,
        "examples": examples,
        "source": text.rstrip(),
        "extras": extras,
        "errors": len(re.findall(r"^\s*// error\[", text, re.M)),
    }


# ---------------------------------------------------------------------------
# Running lessons for their output


def scrub(text):
    text = text.replace(str(ROOT), ".")
    text = text.replace(str(Path.home()), "~")
    text = re.sub(r"/private/var/folders/[^\s\"']+?/T\b", "/tmp", text)
    return text.replace(tempfile.gettempdir().rstrip("/"), "/tmp")


def run_outputs(lessons):
    print("building lessons...", file=sys.stderr)
    subprocess.run(["cargo", "build", "--bins", "-q"], cwd=ROOT, check=True)

    for lesson in lessons:
        exe = ROOT / "target" / "debug" / lesson["slug"]
        try:
            proc = subprocess.run(
                [str(exe)],
                cwd=ROOT,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                timeout=60,
                text=True,
            )
            stdout = scrub(proc.stdout)
        except subprocess.TimeoutExpired:
            print(f"  {lesson['slug']}: timed out", file=sys.stderr)
            stdout = ""

        sections = {}
        current = None
        for line in stdout.splitlines():
            if SECTION.match(line):
                current = line
                sections[current] = []
            elif current:
                sections[current].append(line)

        for ex in lesson["examples"]:
            if len(ex["headers"]) == 1:
                body = sections.get(ex["headers"][0])
                ex["output"] = "\n".join(body) if body is not None else None
            elif ex["headers"]:
                parts = [[h] + sections[h] for h in ex["headers"] if h in sections]
                ex["output"] = "\n".join(sum(parts, [])) or None
            else:
                ex["output"] = None


# ---------------------------------------------------------------------------
# Pages

FONTS = (
    "https://fonts.googleapis.com/css2?family=Inter:opsz,wght@14..32,400..700"
    "&family=JetBrains+Mono:wght@400;500&display=swap"
)

FAVICON = (
    "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'>"
    "<rect width='32' height='32' rx='8' fill='%231C1B19'/><text x='16' y='22' "
    "font-family='Arial' font-weight='700' font-size='17' fill='white' "
    "text-anchor='middle'>B</text></svg>"
)

THEME_BOOT = (
    "(function(){try{var t=localStorage.getItem('_ftheme');"
    "if(!t&&matchMedia('(prefers-color-scheme: dark)').matches)t='dark';"
    "if(t&&t!=='default')document.documentElement.classList.add('theme-'+t)}catch(e){}})();"
)

ICON_COPY = (
    '<svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5">'
    '<rect x="7" y="7" width="10" height="10" rx="2"/>'
    '<path d="M13 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"/></svg>'
)
ICON_PLAY = '<svg viewBox="0 0 20 20" fill="currentColor"><path d="M6 4.5v11l9-5.5z"/></svg>'
ICON_GITHUB = (
    '<svg viewBox="0 0 16 16" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 '
    "3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49"
    "-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82."
    "72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-"
    "1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 "
    "1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 "
    "2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21."
    '15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>'
)


def page(title, body, base, description):
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{html.escape(title)}</title>
<meta name="description" content="{html.escape(description)}">
<link rel="icon" href="{FAVICON}">
<script>{THEME_BOOT}</script>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link rel="stylesheet" href="{FONTS}">
<link rel="stylesheet" href="{base}assets/style.css">
</head>
<body>
<header class="topbar">
  <button class="menu-btn" type="button" aria-label="Lessons" data-menu>
    <svg viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M3 6h14M3 10h14M3 14h14"/></svg>
  </button>
  <a class="brand" href="{base}"><span class="brand-mark">B</span>Buzzard</a>
  <nav class="topnav">
    <a class="nav-link" href="{base}">Lessons</a>
    <a class="nav-link" href="{REPO}" aria-label="GitHub">{ICON_GITHUB}<span>GitHub</span></a>
    <select class="input-field theme-select" aria-label="Theme" data-theme-select>
      <option value="default">Default</option>
      <option value="blue">Blue</option>
      <option value="slate">Slate</option>
      <option value="emerald">Emerald</option>
      <option value="dark">Dark</option>
    </select>
  </nav>
</header>
{body}
<script src="{base}assets/site.js"></script>
</body>
</html>
"""


def code_block(code, extra_buttons=""):
    return (
        '<div class="code">'
        f'<div class="code-actions">{extra_buttons}'
        f'<button class="icon-btn" type="button" data-copy title="Copy">{ICON_COPY}</button>'
        "</div>"
        f"<pre><code>{highlight(code)}</code></pre></div>"
    )


def sidebar(lessons, active, base):
    out = [
        '<aside class="sidebar" data-sidebar>',
        '<input class="input-field side-search" type="search" placeholder="Filter lessons  /" data-filter>',
    ]
    for name, lo, hi in PARTS:
        links = []
        for l in lessons:
            if lo <= l["num"] <= hi:
                cls = "side-link active" if l["slug"] == active else "side-link"
                links.append(
                    f'<a class="{cls}" href="{base}{l["slug"]}/">'
                    f'<span class="side-num">{l["num"]:02}</span>'
                    f'<span>{html.escape(l["title"])}</span></a>'
                )

        out.append(
            f'<div class="side-part"><div class="side-title">{html.escape(name)}</div>'
            + "".join(links)
            + "</div>"
        )

    out.append("</aside>")
    return "\n".join(out)


def part_of(num):
    return next(name for name, lo, hi in PARTS if lo <= num <= hi)


def lesson_page(lesson, prev, nxt, lessons):
    base = "../"
    slug = lesson["slug"]
    commands = "".join(
        f'<div class="run-cmd"><span class="prompt">$</span><code>{html.escape(c)}</code>'
        f'<button class="icon-btn" type="button" data-copy-text="{html.escape(c)}" '
        f'title="Copy">{ICON_COPY}</button></div>'
        for c in lesson["commands"]
    )

    sections = []
    toc = ['<a href="#overview">Overview</a>']
    for ex in lesson["examples"]:
        anchor = f"example-{ex['n']}"
        toc.append(f'<a href="#{anchor}">{html.escape(ex["label"])}</a>')
        output = ""
        if ex.get("output"):
            output = (
                '<div class="output"><div class="output-label">Output</div>'
                f"<pre>{html.escape(ex['output'])}</pre></div>"
            )

        sections.append(
            f'<section class="section" id="{anchor}">'
            f'<h2><a class="anchor" href="#{anchor}">#</a>'
            f'<span class="ex-num">{ex["n"]}</span>{html.escape(ex["label"])}</h2>'
            f"{code_block(ex['code'])}{output}</section>"
        )

    play = ""
    if slug not in NO_PLAYGROUND:
        play = (
            '<button class="btn-secondary btn-sm" type="button" data-play>'
            f"{ICON_PLAY}Run in Playground</button>"
        )

    extras = "".join(
        f'<h3 class="file-name">{html.escape(name)}</h3>{code_block(src)}'
        for name, src in lesson["extras"]
    )
    toc.append('<a href="#full-program">Full program</a>')
    sections.append(
        '<section class="section" id="full-program">'
        '<details data-full><summary><h2><span class="chev"></span>Full program</h2>'
        f'<span class="file-meta">{slug}/main.rs · {lesson["source"].count(chr(10)) + 1} lines</span>'
        "</summary>"
        f'<div class="full-actions">{play}'
        f'<a class="btn-secondary btn-sm" href="{REPO}/blob/main/{slug}/main.rs">{ICON_GITHUB}View on GitHub</a></div>'
        f'<div data-source>{code_block(lesson["source"])}</div>{extras}</details></section>'
    )

    pager = ['<nav class="pager">']
    if prev:
        pager.append(
            f'<a class="pager-card" href="{base}{prev["slug"]}/" data-prev>'
            f'<span class="pager-dir">← Previous</span>'
            f'<span class="pager-title">{prev["num"]:02}. {html.escape(prev["title"])}</span></a>'
        )
    else:
        pager.append("<span></span>")

    if nxt:
        pager.append(
            f'<a class="pager-card next" href="{base}{nxt["slug"]}/" data-next>'
            f'<span class="pager-dir">Next →</span>'
            f'<span class="pager-title">{nxt["num"]:02}. {html.escape(nxt["title"])}</span></a>'
        )

    pager.append("</nav>")

    stats = f'{len(lesson["examples"])} examples'
    if lesson["errors"]:
        plural = "s" if lesson["errors"] > 1 else ""
        stats += f' · {lesson["errors"]} compiler error{plural}'

    body = f"""<div class="layout">
{sidebar(lessons, slug, base)}
<div class="content">
<main class="main">
  <div class="page-header">
    <div class="eyebrow">{html.escape(part_of(lesson["num"]))} · Lesson {lesson["num"]:02}</div>
    <h1 class="page-title">{html.escape(lesson["title"])}</h1>
    <div class="page-subtitle">{stats}</div>
    <div class="commands">{commands}</div>
  </div>
  <section class="section prose" id="overview">{lesson["intro"]}</section>
  {"".join(sections)}
  {"".join(pager)}
</main>
<aside class="toc"><div class="toc-title">On this page</div>{"".join(toc)}</aside>
</div>
</div>"""

    title = f"{lesson['num']:02}. {lesson['title']} · Buzzard"
    return page(title, body, base, f"Buzzard lesson {lesson['num']}: {lesson['title']}")


def index_page(lessons):
    examples = sum(len(l["examples"]) for l in lessons)
    errors = sum(l["errors"] for l in lessons)
    lines = sum(l["source"].count("\n") + 1 for l in lessons)

    parts = []
    for name, lo, hi in PARTS:
        cards = []
        for l in lessons:
            if lo <= l["num"] <= hi:
                cards.append(
                    f'<a class="lesson-card" href="{l["slug"]}/">'
                    f'<span class="lesson-num">{l["num"]:02}</span>'
                    f'<span class="lesson-title">{html.escape(l["title"])}</span>'
                    f'<span class="lesson-meta">{len(l["examples"])} examples</span></a>'
                )

        parts.append(
            f'<section class="part" data-part><h2 class="section-title">{html.escape(name)}'
            f'<span class="part-range">{lo:02}–{hi:02}</span></h2>'
            f'<div class="lesson-grid">{"".join(cards)}</div></section>'
        )

    stat = lambda label, value: (
        f'<div class="stat-card"><div class="stat-label">{label}</div>'
        f'<div class="stat-value">{value:,}</div></div>'
    )

    body = f"""<main class="home">
  <section class="hero">
    <div class="eyebrow">Rust, edition 2024</div>
    <h1 class="hero-title">A hands-on tour of Rust, <br>one small runnable lesson at a time.</h1>
    <p class="hero-sub">Sixty lessons, one topic each. Every lesson opens with an explanation,
    then works through the topic in small examples with the output they print. Code the
    compiler rejects is shown commented out, with the real error it produces.</p>
    <div class="hero-actions">
      <a class="btn-primary" href="{lessons[0]["slug"]}/">Start with lesson 01</a>
      <a class="btn-secondary" href="{REPO}">{ICON_GITHUB}Clone the repo</a>
    </div>
  </section>
  <section class="stats">
    {stat("Lessons", len(lessons))}{stat("Examples", examples)}{stat("Compiler errors explained", errors)}{stat("Lines of Rust", lines)}
  </section>
  <div class="home-search"><input class="input-field" type="search" placeholder="Search lessons  /" data-filter></div>
  {"".join(parts)}
</main>"""

    return page("Buzzard · Learn Rust by example", body, "", "Sixty runnable Rust lessons.")


def not_found_page():
    body = """<main class="home not-found">
  <div class="eyebrow">404</div>
  <h1 class="hero-title">This page does not exist.</h1>
  <p class="hero-sub">The lesson may have been renamed.</p>
  <div class="hero-actions"><a class="btn-primary" data-home href="/">Browse all lessons</a></div>
</main>
<script>document.querySelector('[data-home]').href=location.pathname.split('/').slice(0,2).join('/')+'/';</script>"""
    # The 404 page is served at any depth, so it links assets from the site root.
    return page("Not found · Buzzard", body, "/Buzzard/", "Page not found")


def main():
    lessons = [
        parse(f)
        for f in sorted(ROOT.glob("[0-9][0-9]_*"))
        if (f / "main.rs").is_file()
    ]

    if "--no-run" in sys.argv:
        for l in lessons:
            for ex in l["examples"]:
                ex["output"] = None
    else:
        run_outputs(lessons)

    OUT.mkdir(exist_ok=True)
    for old in OUT.glob("[0-9][0-9]_*"):
        shutil.rmtree(old)

    shutil.rmtree(OUT / "assets", ignore_errors=True)
    (OUT / "assets").mkdir()
    shutil.copy(SITE / "style.css", OUT / "assets" / "style.css")
    shutil.copy(SITE / "site.js", OUT / "assets" / "site.js")
    (OUT / ".nojekyll").write_text("")

    for i, lesson in enumerate(lessons):
        prev = lessons[i - 1] if i > 0 else None
        nxt = lessons[i + 1] if i + 1 < len(lessons) else None
        (OUT / lesson["slug"]).mkdir()
        (OUT / lesson["slug"] / "index.html").write_text(
            lesson_page(lesson, prev, nxt, lessons)
        )

    (OUT / "index.html").write_text(index_page(lessons))
    (OUT / "404.html").write_text(not_found_page())

    missing = [
        f"{l['slug']} #{ex['n']}"
        for l in lessons
        for ex in l["examples"]
        if ex["headers"] and ex.get("output") is None and "--no-run" not in sys.argv
    ]
    if missing:
        print("no output found for: " + ", ".join(missing), file=sys.stderr)

    print(f"wrote {len(lessons)} lessons to {OUT.relative_to(ROOT)}/")


if __name__ == "__main__":
    main()
