// Buzzard docs: theme picker, copy buttons, playground, filtering, and keyboard nav.
(function () {
  var root = document.documentElement;
  var themes = ["blue", "slate", "emerald", "dark"];

  // Theme picker. The choice is stored under the same key Nut uses.
  var select = document.querySelector("[data-theme-select]");
  if (select) {
    var current = themes.find(function (t) {
      return root.classList.contains("theme-" + t);
    });
    select.value = current || "default";

    select.addEventListener("change", function () {
      themes.forEach(function (t) {
        root.classList.remove("theme-" + t);
      });

      if (select.value !== "default") {
        root.classList.add("theme-" + select.value);
      }

      try {
        localStorage.setItem("_ftheme", select.value);
      } catch (e) {}
    });
  }

  // Copy buttons.
  function copy(text, button) {
    var done = function () {
      button.classList.add("copied");
      setTimeout(function () {
        button.classList.remove("copied");
      }, 1200);
    };

    if (navigator.clipboard) {
      navigator.clipboard.writeText(text).then(done, function () {});
    }
  }

  document.addEventListener("click", function (e) {
    var button = e.target.closest("[data-copy], [data-copy-text]");
    if (!button) return;

    var text = button.dataset.copyText;
    if (text === undefined) {
      text = button.closest(".code").querySelector("pre").textContent;
    }

    copy(text, button);
  });

  // Run the full program on play.rust-lang.org. Lessons are too long for a ?code= URL
  // (the playground rejects URLs over ~8 KB), so share it as a gist first, the same
  // way the playground's own Share button does, and open the short ?gist= link.
  var PLAYGROUND = "https://play.rust-lang.org/";
  var gistUrl = null;

  function playgroundCode(play) {
    var actions = play.closest(".code-actions, .full-actions");
    var copyBtn = actions && actions.querySelector("[data-copy-text]");
    if (copyBtn) return copyBtn.dataset.copyText;

    var source = document.querySelector("[data-source] pre");
    return source ? source.textContent : "";
  }

  document.querySelectorAll("[data-play]").forEach(function (play) {
    play.addEventListener("click", function () {
      var code = playgroundCode(play);

      // Open the tab now: browsers block window.open after an await.
      var tab = window.open("about:blank", "_blank");
      if (gistUrl) {
        if (tab) tab.location.href = gistUrl;
        return;
      }

      var label = play.lastChild && play.lastChild.textContent;
      play.disabled = true;
      if (label) play.lastChild.textContent = "Opening…";

      fetch(PLAYGROUND + "meta/gist", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code: code }),
      })
        .then(function (res) {
          if (!res.ok) throw new Error("gist " + res.status);
          return res.json();
        })
        .then(function (gist) {
          gistUrl = PLAYGROUND + "?version=stable&mode=debug&edition=2024&gist=" + gist.id;
          if (tab) tab.location.href = gistUrl;
          else window.location.href = gistUrl;
        })
        .catch(function () {
          // Fall back to an empty playground with the code on the clipboard.
          if (navigator.clipboard) navigator.clipboard.writeText(code);
          if (tab) tab.location.href = PLAYGROUND + "?version=stable&edition=2024";
          alert("Could not share the code with the playground. It is on your clipboard: paste it there.");
        })
        .finally(function () {
          play.disabled = false;
          if (label) play.lastChild.textContent = label;
        });
    });
  });

  // Open the full program when its anchor is targeted.
  var full = document.querySelector("[data-full]");
  if (full && location.hash === "#full-program") full.open = true;
  document.querySelectorAll('a[href="#full-program"]').forEach(function (a) {
    a.addEventListener("click", function () {
      full.open = true;
    });
  });

  // Filter lessons in the sidebar or on the home page.
  var filter = document.querySelector("[data-filter]");
  if (filter) {
    filter.addEventListener("input", function () {
      var q = filter.value.trim().toLowerCase();
      var groups = document.querySelectorAll(".side-part, [data-part]");

      groups.forEach(function (group) {
        var shown = 0;
        group.querySelectorAll(".side-link, .lesson-card").forEach(function (link) {
          var hit = !q || link.textContent.toLowerCase().indexOf(q) !== -1;
          link.hidden = !hit;
          if (hit) shown++;
        });

        group.hidden = shown === 0;
      });
    });

    filter.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        var first = document.querySelector(".side-link:not([hidden]), .lesson-card:not([hidden])");
        if (first) location.href = first.href;
      } else if (e.key === "Escape") {
        filter.value = "";
        filter.dispatchEvent(new Event("input"));
        filter.blur();
      }
    });
  }

  // Mobile sidebar.
  var menu = document.querySelector("[data-menu]");
  var sidebar = document.querySelector("[data-sidebar]");
  if (menu && sidebar) {
    menu.addEventListener("click", function () {
      sidebar.classList.toggle("open");
    });
  } else if (menu) {
    menu.hidden = true;
  }

  // Keep the active sidebar link in view.
  var active = document.querySelector(".side-link.active");
  if (active && sidebar) {
    sidebar.scrollTop = active.offsetTop - sidebar.clientHeight / 3;
  }

  // Highlight the section being read in "On this page".
  var tocLinks = document.querySelectorAll(".toc a");
  if (tocLinks.length && "IntersectionObserver" in window) {
    var byId = {};
    tocLinks.forEach(function (a) {
      byId[a.getAttribute("href").slice(1)] = a;
    });

    var observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (!entry.isIntersecting) return;
          tocLinks.forEach(function (a) {
            a.classList.remove("active");
          });
          byId[entry.target.id].classList.add("active");
        });
      },
      { rootMargin: "-80px 0px -65% 0px" }
    );

    Object.keys(byId).forEach(function (id) {
      var el = document.getElementById(id);
      if (el) observer.observe(el);
    });
  }

  // Keyboard: "/" focuses the filter, left and right arrows move between lessons.
  document.addEventListener("keydown", function (e) {
    var tag = (e.target.tagName || "").toLowerCase();
    if (tag === "input" || tag === "select" || tag === "textarea") return;
    if (e.metaKey || e.ctrlKey || e.altKey) return;

    if (e.key === "/" && filter) {
      e.preventDefault();
      filter.focus();
    } else if (e.key === "ArrowLeft" || e.key === "ArrowRight") {
      var link = document.querySelector(e.key === "ArrowLeft" ? "[data-prev]" : "[data-next]");
      if (link) location.href = link.href;
    }
  });
})();
