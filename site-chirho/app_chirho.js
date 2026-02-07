// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

(function () {
  "use strict";

  var DATA_DIR_CHIRHO = "data-chirho";
  var VERSES_PER_CHUNK_CHIRHO = 5;

  var manifestChirho = null;
  var renderCancelChirho = null;
  var loadGenChirho = 0;

  var bookListElChirho = document.getElementById("book-list-chirho");
  var chapterTitleElChirho = document.getElementById("chapter-title-chirho");
  var chapterNavElChirho = document.getElementById("chapter-nav-chirho");
  var textAreaElChirho = document.getElementById("text-area-chirho");
  var tooltipElChirho = document.getElementById("tooltip-chirho");
  var statsBarElChirho = document.getElementById("stats-bar-chirho");
  var menuToggleElChirho = document.getElementById("menu-toggle-chirho");
  var sidebarElChirho = document.getElementById("sidebar-chirho");
  var homeLinkElChirho = document.getElementById("home-link-chirho");

  menuToggleElChirho.addEventListener("click", function () {
    sidebarElChirho.classList.toggle("open-chirho");
  });

  // Sidebar title goes home
  homeLinkElChirho.addEventListener("click", function (eChirho) {
    eChirho.preventDefault();
    cancelRenderChirho();
    chapterTitleElChirho.textContent = "";
    chapterNavElChirho.textContent = "";
    window.location.hash = "";
    document.querySelectorAll(".book-item-chirho").forEach(function (elChirho) {
      elChirho.classList.remove("active-chirho");
    });
    showFrontPageChirho();
    sidebarElChirho.classList.remove("open-chirho");
  });

  function cancelRenderChirho() {
    if (renderCancelChirho) {
      renderCancelChirho.cancelledChirho = true;
      renderCancelChirho = null;
    }
  }

  function showFrontPageChirho() {
    textAreaElChirho.textContent = "";
    var divChirho = document.createElement("div");
    divChirho.className = "placeholder-chirho";

    var p1Chirho = document.createElement("p");
    var strong1Chirho = document.createElement("strong");
    strong1Chirho.textContent = "Mikra according to the Masorah";
    var strong2Chirho = document.createElement("strong");
    strong2Chirho.textContent = "Solid Rock Hebrew Bible";
    p1Chirho.appendChild(document.createTextNode("Cantillation accents (te\u2019amim) from the "));
    p1Chirho.appendChild(strong1Chirho);
    p1Chirho.appendChild(document.createTextNode(" (MapM, based on the Aleppo Codex) transposed onto the "));
    p1Chirho.appendChild(strong2Chirho);
    p1Chirho.appendChild(document.createTextNode(" (based on the Leningrad Codex)."));
    divChirho.appendChild(p1Chirho);

    var p2Chirho = document.createElement("p");
    p2Chirho.style.marginTop = "0.8rem";
    p2Chirho.textContent = "The Solid Rock Hebrew Bible is a TEI XML critical edition of the Leningrad Codex. MapM preserves the cantillation tradition of the Aleppo Codex. This viewer shows each word color-coded by how well the two traditions align.";
    divChirho.appendChild(p2Chirho);

    var p3Chirho = document.createElement("p");
    p3Chirho.style.marginTop = "0.8rem";
    p3Chirho.textContent = "Select a book and chapter from the sidebar to begin.";
    divChirho.appendChild(p3Chirho);

    var p4Chirho = document.createElement("p");
    p4Chirho.style.marginTop = "1.2rem";
    var ghLinkChirho = document.createElement("a");
    ghLinkChirho.href = "https://github.com/loveJesus/accent-transpose-chirho";
    ghLinkChirho.target = "_blank";
    ghLinkChirho.rel = "noopener";
    ghLinkChirho.style.color = "#7c8cf8";
    ghLinkChirho.textContent = "GitHub: loveJesus/accent-transpose-chirho";
    p4Chirho.appendChild(ghLinkChirho);
    divChirho.appendChild(p4Chirho);

    textAreaElChirho.appendChild(divChirho);
  }

  async function loadManifestChirho() {
    var respChirho = await fetch(DATA_DIR_CHIRHO + "/manifest_chirho.json");
    manifestChirho = await respChirho.json();
    renderStatsChirho(manifestChirho.total_stats_chirho);
    renderBookListChirho();

    if (window.location.hash && window.location.hash.length > 1) {
      parseHashChirho();
    } else {
      showFrontPageChirho();
    }
  }

  function renderStatsChirho(statsChirho) {
    statsBarElChirho.textContent = "";
    var strongChirho = document.createElement("strong");
    strongChirho.textContent = statsChirho.total.toLocaleString();
    statsBarElChirho.appendChild(strongChirho);
    statsBarElChirho.appendChild(document.createTextNode(" words"));
    statsBarElChirho.appendChild(document.createElement("br"));

    var pctChirho = function (nChirho) {
      return ((nChirho / statsChirho.total) * 100).toFixed(1);
    };

    var matchedSpanChirho = document.createElement("span");
    matchedSpanChirho.style.color = "#2e7d32";
    matchedSpanChirho.textContent = pctChirho(statsChirho.unmodified) + "% matched";
    statsBarElChirho.appendChild(matchedSpanChirho);
    statsBarElChirho.appendChild(document.createTextNode(" \u00b7 "));

    var mismatchSpanChirho = document.createElement("span");
    mismatchSpanChirho.style.color = "#e65100";
    mismatchSpanChirho.textContent = pctChirho(statsChirho.mismatch) + "% mismatch";
    statsBarElChirho.appendChild(mismatchSpanChirho);
    statsBarElChirho.appendChild(document.createTextNode(" \u00b7 "));

    var missingSpanChirho = document.createElement("span");
    missingSpanChirho.style.color = "#c62828";
    missingSpanChirho.textContent = pctChirho(statsChirho.not_present) + "% missing";
    statsBarElChirho.appendChild(missingSpanChirho);
  }

  function renderBookListChirho() {
    bookListElChirho.textContent = "";
    manifestChirho.books_chirho.forEach(function (bookChirho) {
      var divChirho = document.createElement("div");
      divChirho.className = "book-item-chirho";
      divChirho.textContent = bookChirho.book_name_chirho;
      divChirho.dataset.bookNum = bookChirho.book_num_chirho;
      divChirho.addEventListener("click", function () {
        selectBookChirho(bookChirho);
      });
      bookListElChirho.appendChild(divChirho);
    });
  }

  function selectBookChirho(bookChirho) {
    document.querySelectorAll(".book-item-chirho").forEach(function (elChirho) {
      elChirho.classList.toggle(
        "active-chirho",
        parseInt(elChirho.dataset.bookNum) === bookChirho.book_num_chirho
      );
    });

    chapterNavElChirho.textContent = "";
    bookChirho.chapters_chirho.forEach(function (chChirho) {
      var btnChirho = document.createElement("button");
      btnChirho.className = "chapter-btn-chirho";
      btnChirho.textContent = chChirho.chapter_chirho;
      btnChirho.addEventListener("click", function () {
        loadChapterChirho(bookChirho, chChirho);
      });
      chapterNavElChirho.appendChild(btnChirho);
    });

    if (bookChirho.chapters_chirho.length > 0) {
      loadChapterChirho(bookChirho, bookChirho.chapters_chirho[0]);
    }

    sidebarElChirho.classList.remove("open-chirho");
  }

  async function loadChapterChirho(bookChirho, chChirho) {
    cancelRenderChirho();
    var genChirho = ++loadGenChirho;

    chapterTitleElChirho.textContent = bookChirho.book_name_chirho + " " + chChirho.chapter_chirho;
    window.location.hash = bookChirho.book_num_chirho + ":" + chChirho.chapter_chirho;

    document.querySelectorAll(".chapter-btn-chirho").forEach(function (btnChirho) {
      btnChirho.classList.toggle(
        "active-chirho",
        parseInt(btnChirho.textContent) === chChirho.chapter_chirho
      );
    });

    textAreaElChirho.textContent = "";
    var loadingPChirho = document.createElement("p");
    loadingPChirho.className = "placeholder-chirho";
    loadingPChirho.textContent = "Loading...";
    textAreaElChirho.appendChild(loadingPChirho);

    var respChirho = await fetch(DATA_DIR_CHIRHO + "/" + chChirho.file_chirho);
    if (genChirho !== loadGenChirho) return;
    var dataChirho = await respChirho.json();
    if (genChirho !== loadGenChirho) return;

    renderChapterChunkedChirho(dataChirho);
  }

  var STATUS_CLASSES_CHIRHO = {
    unmodified: "word-unmodified-chirho",
    cantillated: "word-unmodified-chirho",
    mismatch: "word-mismatch-chirho",
    not_present_in_mapm: "word-not-present-chirho"
  };

  // Render a single verse into a DocumentFragment
  function renderVerseChirho(verseChirho) {
    var fragChirho = document.createDocumentFragment();
    var numSpanChirho = document.createElement("span");
    numSpanChirho.className = "verse-num-chirho";
    numSpanChirho.textContent = verseChirho.verse_chirho;
    fragChirho.appendChild(numSpanChirho);

    for (var iChirho = 0; iChirho < verseChirho.words_chirho.length; iChirho++) {
      var wordChirho = verseChirho.words_chirho[iChirho];
      var spanChirho = document.createElement("span");
      spanChirho.className = "word-chirho " + (STATUS_CLASSES_CHIRHO[wordChirho.status_chirho] || "word-not-present-chirho");
      spanChirho.textContent = wordChirho.result_chirho;
      spanChirho.dataset.o = wordChirho.sr_original_chirho;
      spanChirho.dataset.r = wordChirho.result_chirho;
      spanChirho.dataset.s = wordChirho.status_chirho;
      spanChirho.dataset.c = wordChirho.confidence_chirho;
      if (wordChirho.notes_chirho) {
        spanChirho.dataset.n = wordChirho.notes_chirho;
      }
      fragChirho.appendChild(document.createTextNode(" "));
      fragChirho.appendChild(spanChirho);
    }

    fragChirho.appendChild(document.createTextNode(" "));
    return fragChirho;
  }

  // Chunked rendering: render VERSES_PER_CHUNK_CHIRHO verses per animation frame
  function renderChapterChunkedChirho(dataChirho) {
    cancelRenderChirho();
    textAreaElChirho.textContent = "";

    // Stats bar
    var statsDivChirho = document.createElement("div");
    statsDivChirho.className = "chapter-stats-chirho";
    statsDivChirho.textContent =
      dataChirho.stats_chirho.total + " words: " +
      dataChirho.stats_chirho.unmodified + " matched, " +
      dataChirho.stats_chirho.mismatch + " mismatch, " +
      dataChirho.stats_chirho.not_present + " missing";
    textAreaElChirho.appendChild(statsDivChirho);

    var containerChirho = document.createElement("div");
    containerChirho.style.marginTop = "1rem";
    textAreaElChirho.appendChild(containerChirho);

    // Event delegation on container
    containerChirho.addEventListener("mouseenter", handleWordHoverChirho, true);
    containerChirho.addEventListener("mouseleave", handleWordLeaveChirho, true);
    containerChirho.addEventListener("click", handleWordClickChirho, true);

    var versesChirho = dataChirho.verses_chirho;
    var idxChirho = 0;
    var tokenChirho = { cancelledChirho: false };
    renderCancelChirho = tokenChirho;

    function renderNextChunkChirho() {
      if (tokenChirho.cancelledChirho) return;

      var endChirho = Math.min(idxChirho + VERSES_PER_CHUNK_CHIRHO, versesChirho.length);
      var fragChirho = document.createDocumentFragment();
      while (idxChirho < endChirho) {
        fragChirho.appendChild(renderVerseChirho(versesChirho[idxChirho]));
        idxChirho++;
      }
      containerChirho.appendChild(fragChirho);

      if (idxChirho < versesChirho.length) {
        requestAnimationFrame(renderNextChunkChirho);
      } else {
        renderCancelChirho = null;
      }
    }

    requestAnimationFrame(renderNextChunkChirho);
  }

  function handleWordHoverChirho(eChirho) {
    if (eChirho.target.classList.contains("word-chirho")) {
      showTooltipChirho(eChirho.target, eChirho);
    }
  }

  function handleWordLeaveChirho(eChirho) {
    if (eChirho.target.classList.contains("word-chirho")) {
      hideTooltipChirho();
    }
  }

  function handleWordClickChirho(eChirho) {
    if (eChirho.target.classList.contains("word-chirho")) {
      if (tooltipElChirho.style.display === "block") {
        hideTooltipChirho();
      } else {
        showTooltipChirho(eChirho.target, eChirho);
      }
    }
  }

  var STATUS_LABELS_CHIRHO = {
    unmodified: "Matched",
    cantillated: "Matched",
    mismatch: "Mismatch",
    not_present_in_mapm: "Not in MapM"
  };

  function showTooltipChirho(elChirho, eChirho) {
    tooltipElChirho.textContent = "";
    appendTooltipRowChirho("Status", (STATUS_LABELS_CHIRHO[elChirho.dataset.s] || elChirho.dataset.s) + " (confidence: " + elChirho.dataset.c + ")", false);
    appendTooltipRowChirho("SR Original", elChirho.dataset.o, true);
    appendTooltipRowChirho("Result", elChirho.dataset.r, true);
    if (elChirho.dataset.n) {
      appendTooltipRowChirho("Notes", elChirho.dataset.n, false);
    }
    tooltipElChirho.style.display = "block";
    positionTooltipChirho(eChirho);
  }

  function appendTooltipRowChirho(labelChirho, valueChirho, isHebrewChirho) {
    var labelDivChirho = document.createElement("div");
    labelDivChirho.className = "tip-label-chirho";
    if (tooltipElChirho.children.length > 0) {
      labelDivChirho.style.marginTop = "0.4rem";
    }
    labelDivChirho.textContent = labelChirho;
    tooltipElChirho.appendChild(labelDivChirho);

    var valueDivChirho = document.createElement("div");
    if (isHebrewChirho) {
      valueDivChirho.className = "tip-hebrew-chirho";
    }
    valueDivChirho.textContent = valueChirho;
    tooltipElChirho.appendChild(valueDivChirho);
  }

  function positionTooltipChirho(eChirho) {
    var xChirho = eChirho.clientX + 12;
    var yChirho = eChirho.clientY + 12;
    var wChirho = tooltipElChirho.offsetWidth;
    var hChirho = tooltipElChirho.offsetHeight;
    if (xChirho + wChirho > window.innerWidth - 10) xChirho = eChirho.clientX - wChirho - 12;
    if (yChirho + hChirho > window.innerHeight - 10) yChirho = eChirho.clientY - hChirho - 12;
    tooltipElChirho.style.left = xChirho + "px";
    tooltipElChirho.style.top = yChirho + "px";
  }

  function hideTooltipChirho() {
    tooltipElChirho.style.display = "none";
  }

  function parseHashChirho() {
    var hashChirho = window.location.hash.slice(1);
    var partsChirho = hashChirho.split(":");
    if (partsChirho.length === 2) {
      var bookNumChirho = parseInt(partsChirho[0]);
      var chapterNumChirho = parseInt(partsChirho[1]);
      var bookChirho = manifestChirho.books_chirho.find(function (bChirho) {
        return bChirho.book_num_chirho === bookNumChirho;
      });
      if (bookChirho) {
        selectBookChirho(bookChirho);
        var chChirho = bookChirho.chapters_chirho.find(function (cChirho) {
          return cChirho.chapter_chirho === chapterNumChirho;
        });
        if (chChirho) {
          loadChapterChirho(bookChirho, chChirho);
        }
      }
    }
  }

  window.addEventListener("hashchange", function () {
    if (manifestChirho) {
      if (!window.location.hash || window.location.hash === "#") {
        cancelRenderChirho();
        showFrontPageChirho();
      } else {
        parseHashChirho();
      }
    }
  });

  loadManifestChirho();
})();
