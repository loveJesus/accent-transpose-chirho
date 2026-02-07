// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

(function () {
  "use strict";

  var DATA_DIR_CHIRHO = "data-chirho";
  var PDF_URL_CHIRHO = "https://media-solid-rock-accents-chirho.bible.systems/bible_chirho.pdf";

  var manifestChirho = null;
  var currentBookChirho = null;
  var currentChapterChirho = null;

  var bookListEl = document.getElementById("book-list-chirho");
  var chapterTitleEl = document.getElementById("chapter-title-chirho");
  var chapterNavEl = document.getElementById("chapter-nav-chirho");
  var textAreaEl = document.getElementById("text-area-chirho");
  var tooltipEl = document.getElementById("tooltip-chirho");
  var statsBarEl = document.getElementById("stats-bar-chirho");
  var menuToggleEl = document.getElementById("menu-toggle-chirho");
  var sidebarEl = document.getElementById("sidebar-chirho");
  var homeLinkEl = document.getElementById("home-link-chirho");

  // Mobile menu toggle
  menuToggleEl.addEventListener("click", function () {
    sidebarEl.classList.toggle("open-chirho");
  });

  // Home link returns to front page
  homeLinkEl.addEventListener("click", function (e) {
    e.preventDefault();
    currentBookChirho = null;
    currentChapterChirho = null;
    chapterTitleEl.textContent = "";
    chapterNavEl.textContent = "";
    window.location.hash = "";
    document.querySelectorAll(".book-item-chirho").forEach(function (el) {
      el.classList.remove("active-chirho");
    });
    showFrontPageChirho();
  });

  function showFrontPageChirho() {
    textAreaEl.textContent = "";
    var div = document.createElement("div");
    div.className = "placeholder-chirho";

    var p1 = document.createElement("p");
    var strong1 = document.createElement("strong");
    strong1.textContent = "Mikra according to the Masorah";
    var strong2 = document.createElement("strong");
    strong2.textContent = "Solid Rock Hebrew Bible";
    p1.appendChild(document.createTextNode("Cantillation accents (te\u2019amim) from the "));
    p1.appendChild(strong1);
    p1.appendChild(document.createTextNode(" (MapM, based on the Aleppo Codex) transposed onto the "));
    p1.appendChild(strong2);
    p1.appendChild(document.createTextNode(" (based on the Leningrad Codex)."));
    div.appendChild(p1);

    var p2 = document.createElement("p");
    p2.style.marginTop = "0.8rem";
    p2.textContent = "The Solid Rock Hebrew Bible is a TEI XML critical edition of the Leningrad Codex. MapM preserves the cantillation tradition of the Aleppo Codex. This viewer shows each word color-coded by how well the two traditions align.";
    div.appendChild(p2);

    var p3 = document.createElement("p");
    p3.style.marginTop = "0.8rem";
    p3.textContent = "Select a book and chapter from the sidebar to begin.";
    div.appendChild(p3);

    var p4 = document.createElement("p");
    p4.style.marginTop = "1.2rem";
    var ghLink = document.createElement("a");
    ghLink.href = "https://github.com/loveJesus/accent-transpose-chirho";
    ghLink.target = "_blank";
    ghLink.rel = "noopener";
    ghLink.style.color = "#7c8cf8";
    ghLink.textContent = "GitHub: loveJesus/accent-transpose-chirho";
    p4.appendChild(ghLink);

    var pdfSpan = document.createTextNode(" \u00b7 ");
    p4.appendChild(pdfSpan);
    var pdfLink = document.createElement("a");
    pdfLink.href = PDF_URL_CHIRHO;
    pdfLink.target = "_blank";
    pdfLink.rel = "noopener";
    pdfLink.style.color = "#7c8cf8";
    pdfLink.textContent = "Download PDF";
    p4.appendChild(pdfLink);

    div.appendChild(p4);
    textAreaEl.appendChild(div);
  }

  async function loadManifestChirho() {
    var resp = await fetch(DATA_DIR_CHIRHO + "/manifest_chirho.json");
    manifestChirho = await resp.json();
    renderStatsChirho(manifestChirho.total_stats_chirho);
    renderBookListChirho();

    // Check URL hash for deep linking
    if (window.location.hash && window.location.hash.length > 1) {
      parseHashChirho();
    } else {
      showFrontPageChirho();
    }
  }

  function renderStatsChirho(stats) {
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

    document.querySelectorAll(".book-item-chirho").forEach(function (el) {
      el.classList.toggle(
        "active-chirho",
        parseInt(el.dataset.bookNum) === book.book_num_chirho
      );
    });

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

    if (book.chapters_chirho.length > 0) {
      loadChapterChirho(book, book.chapters_chirho[0]);
    }

    sidebarEl.classList.remove("open-chirho");
  }

  async function loadChapterChirho(book, ch) {
    currentChapterChirho = ch;
    chapterTitleEl.textContent = book.book_name_chirho + " " + ch.chapter_chirho;

    window.location.hash = book.book_num_chirho + ":" + ch.chapter_chirho;

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

  var STATUS_CLASSES_CHIRHO = {
    unmodified: "word-unmodified-chirho",
    cantillated: "word-unmodified-chirho",
    mismatch: "word-mismatch-chirho",
    not_present_in_mapm: "word-not-present-chirho"
  };

  function renderChapterChirho(data) {
    textAreaEl.textContent = "";

    // Chapter stats
    var statsDiv = document.createElement("div");
    statsDiv.className = "chapter-stats-chirho";
    statsDiv.textContent =
      data.stats_chirho.total + " words: " +
      data.stats_chirho.unmodified + " matched, " +
      data.stats_chirho.mismatch + " mismatch, " +
      data.stats_chirho.not_present + " missing";
    textAreaEl.appendChild(statsDiv);

    // Use DocumentFragment for fast batch DOM insertion
    var frag = document.createDocumentFragment();
    var container = document.createElement("div");
    container.className = "verses-container-chirho";
    container.style.marginTop = "1rem";

    data.verses_chirho.forEach(function (verse) {
      var numSpan = document.createElement("span");
      numSpan.className = "verse-num-chirho";
      numSpan.textContent = verse.verse_chirho;
      container.appendChild(numSpan);

      verse.words_chirho.forEach(function (word) {
        var span = document.createElement("span");
        span.className = "word-chirho " + (STATUS_CLASSES_CHIRHO[word.status_chirho] || "word-not-present-chirho");
        span.textContent = word.result_chirho;
        // Store data for tooltip via data attributes
        span.dataset.o = word.sr_original_chirho;
        span.dataset.r = word.result_chirho;
        span.dataset.s = word.status_chirho;
        span.dataset.c = word.confidence_chirho;
        if (word.notes_chirho) {
          span.dataset.n = word.notes_chirho;
        }
        container.appendChild(document.createTextNode(" "));
        container.appendChild(span);
      });

      container.appendChild(document.createTextNode(" "));
    });

    frag.appendChild(container);

    // Event delegation: single listener on container for all words
    container.addEventListener("mouseenter", handleWordHoverChirho, true);
    container.addEventListener("mouseleave", handleWordLeaveChirho, true);
    container.addEventListener("click", handleWordClickChirho, true);

    textAreaEl.appendChild(frag);
  }

  function handleWordHoverChirho(e) {
    if (e.target.classList.contains("word-chirho")) {
      showTooltipChirho(e.target, e);
    }
  }

  function handleWordLeaveChirho(e) {
    if (e.target.classList.contains("word-chirho")) {
      hideTooltipChirho();
    }
  }

  function handleWordClickChirho(e) {
    if (e.target.classList.contains("word-chirho")) {
      if (tooltipEl.style.display === "block") {
        hideTooltipChirho();
      } else {
        showTooltipChirho(e.target, e);
      }
    }
  }

  var STATUS_LABELS_CHIRHO = {
    unmodified: "Matched",
    cantillated: "Matched",
    mismatch: "Mismatch",
    not_present_in_mapm: "Not in MapM"
  };

  function showTooltipChirho(el, e) {
    tooltipEl.textContent = "";

    appendTooltipRowChirho("Status", (STATUS_LABELS_CHIRHO[el.dataset.s] || el.dataset.s) + " (confidence: " + el.dataset.c + ")", false);
    appendTooltipRowChirho("SR Original", el.dataset.o, true);
    appendTooltipRowChirho("Result", el.dataset.r, true);

    if (el.dataset.n) {
      appendTooltipRowChirho("Notes", el.dataset.n, false);
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
      if (!window.location.hash || window.location.hash === "#") {
        showFrontPageChirho();
      } else {
        parseHashChirho();
      }
    }
  });

  loadManifestChirho();
})();
