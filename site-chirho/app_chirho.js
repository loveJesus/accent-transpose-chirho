// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

(function () {
  "use strict";

  const DATA_DIR_CHIRHO = "data-chirho";

  let manifestChirho = null;
  let currentBookChirho = null;
  let currentChapterChirho = null;

  const bookListEl = document.getElementById("book-list-chirho");
  const chapterTitleEl = document.getElementById("chapter-title-chirho");
  const chapterNavEl = document.getElementById("chapter-nav-chirho");
  const textAreaEl = document.getElementById("text-area-chirho");
  const tooltipEl = document.getElementById("tooltip-chirho");
  const statsBarEl = document.getElementById("stats-bar-chirho");
  const menuToggleEl = document.getElementById("menu-toggle-chirho");
  const sidebarEl = document.getElementById("sidebar-chirho");

  // Mobile menu toggle
  menuToggleEl.addEventListener("click", function () {
    sidebarEl.classList.toggle("open-chirho");
  });

  async function loadManifestChirho() {
    const resp = await fetch(DATA_DIR_CHIRHO + "/manifest_chirho.json");
    manifestChirho = await resp.json();
    renderStatsChirho(manifestChirho.total_stats_chirho);
    renderBookListChirho();

    // Check URL hash for deep linking
    if (window.location.hash) {
      parseHashChirho();
    }
  }

  function renderStatsChirho(stats) {
    // Build stats bar using safe DOM methods
    statsBarEl.textContent = "";
    var strong = document.createElement("strong");
    strong.textContent = stats.total.toLocaleString();
    statsBarEl.appendChild(strong);
    statsBarEl.appendChild(document.createTextNode(" words"));
    statsBarEl.appendChild(document.createElement("br"));

    var pct = function (n) {
      return ((n / stats.total) * 100).toFixed(1);
    };

    var matchedSpan = document.createElement("span");
    matchedSpan.style.color = "#2e7d32";
    matchedSpan.textContent = pct(stats.unmodified) + "% matched";
    statsBarEl.appendChild(matchedSpan);
    statsBarEl.appendChild(document.createTextNode(" \u00b7 "));

    var mismatchSpan = document.createElement("span");
    mismatchSpan.style.color = "#e65100";
    mismatchSpan.textContent = pct(stats.mismatch) + "% mismatch";
    statsBarEl.appendChild(mismatchSpan);
    statsBarEl.appendChild(document.createTextNode(" \u00b7 "));

    var missingSpan = document.createElement("span");
    missingSpan.style.color = "#c62828";
    missingSpan.textContent = pct(stats.not_present) + "% missing";
    statsBarEl.appendChild(missingSpan);
  }

  function renderBookListChirho() {
    bookListEl.textContent = "";
    manifestChirho.books_chirho.forEach(function (book) {
      var div = document.createElement("div");
      div.className = "book-item-chirho";
      div.textContent = book.book_name_chirho;
      div.dataset.bookNum = book.book_num_chirho;
      div.addEventListener("click", function () {
        selectBookChirho(book);
      });
      bookListEl.appendChild(div);
    });
  }

  function selectBookChirho(book) {
    currentBookChirho = book;

    // Highlight active book
    document.querySelectorAll(".book-item-chirho").forEach(function (el) {
      el.classList.toggle(
        "active-chirho",
        parseInt(el.dataset.bookNum) === book.book_num_chirho
      );
    });

    // Build chapter navigation
    chapterNavEl.textContent = "";
    book.chapters_chirho.forEach(function (ch) {
      var btn = document.createElement("button");
      btn.className = "chapter-btn-chirho";
      btn.textContent = ch.chapter_chirho;
      btn.addEventListener("click", function () {
        loadChapterChirho(book, ch);
      });
      chapterNavEl.appendChild(btn);
    });

    // Auto-load chapter 1
    if (book.chapters_chirho.length > 0) {
      loadChapterChirho(book, book.chapters_chirho[0]);
    }

    // Close mobile menu
    sidebarEl.classList.remove("open-chirho");
  }

  async function loadChapterChirho(book, ch) {
    currentChapterChirho = ch;
    chapterTitleEl.textContent = book.book_name_chirho + " " + ch.chapter_chirho;

    // Update hash for deep linking
    window.location.hash = book.book_num_chirho + ":" + ch.chapter_chirho;

    // Highlight active chapter button
    document.querySelectorAll(".chapter-btn-chirho").forEach(function (btn) {
      btn.classList.toggle(
        "active-chirho",
        parseInt(btn.textContent) === ch.chapter_chirho
      );
    });

    textAreaEl.textContent = "";
    var loadingP = document.createElement("p");
    loadingP.className = "placeholder-chirho";
    loadingP.textContent = "Loading...";
    textAreaEl.appendChild(loadingP);

    var resp = await fetch(DATA_DIR_CHIRHO + "/" + ch.file_chirho);
    var data = await resp.json();

    renderChapterChirho(data);
  }

  function statusClassChirho(status) {
    switch (status) {
      case "unmodified":
      case "cantillated":
        return "word-unmodified-chirho";
      case "mismatch":
        return "word-mismatch-chirho";
      default:
        return "word-not-present-chirho";
    }
  }

  function renderChapterChirho(data) {
    textAreaEl.textContent = "";

    // Chapter stats bar
    var statsDiv = document.createElement("div");
    statsDiv.className = "chapter-stats-chirho";
    statsDiv.textContent =
      data.stats_chirho.total + " words: " +
      data.stats_chirho.unmodified + " matched, " +
      data.stats_chirho.mismatch + " mismatch, " +
      data.stats_chirho.not_present + " missing";
    textAreaEl.appendChild(statsDiv);

    var container = document.createElement("div");
    container.style.marginTop = "1rem";

    data.verses_chirho.forEach(function (verse) {
      // Verse number
      var numSpan = document.createElement("span");
      numSpan.className = "verse-num-chirho";
      numSpan.textContent = verse.verse_chirho;
      container.appendChild(numSpan);

      verse.words_chirho.forEach(function (word) {
        var span = document.createElement("span");
        span.className = "word-chirho " + statusClassChirho(word.status_chirho);
        span.textContent = word.result_chirho;
        span.dataset.srOriginal = word.sr_original_chirho;
        span.dataset.result = word.result_chirho;
        span.dataset.status = word.status_chirho;
        span.dataset.confidence = word.confidence_chirho;
        span.dataset.notes = word.notes_chirho || "";

        span.addEventListener("mouseenter", showTooltipChirho);
        span.addEventListener("mouseleave", hideTooltipChirho);
        span.addEventListener("click", toggleTooltipMobileChirho);

        container.appendChild(document.createTextNode(" "));
        container.appendChild(span);
      });

      container.appendChild(document.createTextNode(" "));
    });

    textAreaEl.appendChild(container);
  }

  function showTooltipChirho(e) {
    var el = e.currentTarget;
    var status = el.dataset.status;
    var statusLabel =
      status === "unmodified" || status === "cantillated"
        ? "Matched"
        : status === "mismatch"
        ? "Mismatch"
        : "Not in MapM";

    // Build tooltip using safe DOM methods
    tooltipEl.textContent = "";

    appendTooltipRowChirho("Status", statusLabel + " (confidence: " + el.dataset.confidence + ")", false);
    appendTooltipRowChirho("SR Original", el.dataset.srOriginal, true);
    appendTooltipRowChirho("Result", el.dataset.result, true);

    if (el.dataset.notes) {
      appendTooltipRowChirho("Notes", el.dataset.notes, false);
    }

    tooltipEl.style.display = "block";
    positionTooltipChirho(e);
  }

  function appendTooltipRowChirho(label, value, isHebrew) {
    var labelDiv = document.createElement("div");
    labelDiv.className = "tip-label-chirho";
    if (tooltipEl.children.length > 0) {
      labelDiv.style.marginTop = "0.4rem";
    }
    labelDiv.textContent = label;
    tooltipEl.appendChild(labelDiv);

    var valueDiv = document.createElement("div");
    if (isHebrew) {
      valueDiv.className = "tip-hebrew-chirho";
    }
    valueDiv.textContent = value;
    tooltipEl.appendChild(valueDiv);
  }

  function positionTooltipChirho(e) {
    var x = e.clientX + 12;
    var y = e.clientY + 12;
    var w = tooltipEl.offsetWidth;
    var h = tooltipEl.offsetHeight;

    if (x + w > window.innerWidth - 10) {
      x = e.clientX - w - 12;
    }
    if (y + h > window.innerHeight - 10) {
      y = e.clientY - h - 12;
    }

    tooltipEl.style.left = x + "px";
    tooltipEl.style.top = y + "px";
  }

  function hideTooltipChirho() {
    tooltipEl.style.display = "none";
  }

  function toggleTooltipMobileChirho(e) {
    if (tooltipEl.style.display === "block") {
      hideTooltipChirho();
    } else {
      showTooltipChirho(e);
    }
  }

  function parseHashChirho() {
    var hash = window.location.hash.slice(1);
    var parts = hash.split(":");
    if (parts.length === 2) {
      var bookNum = parseInt(parts[0]);
      var chapterNum = parseInt(parts[1]);

      var book = manifestChirho.books_chirho.find(function (b) {
        return b.book_num_chirho === bookNum;
      });
      if (book) {
        selectBookChirho(book);
        var ch = book.chapters_chirho.find(function (c) {
          return c.chapter_chirho === chapterNum;
        });
        if (ch) {
          loadChapterChirho(book, ch);
        }
      }
    }
  }

  window.addEventListener("hashchange", function () {
    if (manifestChirho) {
      parseHashChirho();
    }
  });

  loadManifestChirho();
})();
