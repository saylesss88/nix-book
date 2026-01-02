document.addEventListener("DOMContentLoaded", () => {
  const menuBar =
    document.querySelector(".menu-bar .right-buttons") ||
    document.querySelector(".menu-bar");
  if (!menuBar) return;

  const rssLink = document.createElement("a");
  rssLink.href = "https://your-user.github.io/rss.xml"; // set to your feed URL
  rssLink.target = "_blank";
  rssLink.rel = "noopener";
  rssLink.title = "Subscribe to RSS feed";
  rssLink.className = "rss-btn";

  rssLink.innerHTML = `
    <svg xmlns="http://www.w3.org/2000/svg"
         width="16" height="16" viewBox="0 0 24 24"
         fill="none" stroke="currentColor" stroke-width="2"
         stroke-linecap="round" stroke-linejoin="round"
         style="margin-bottom:-3px">
      <circle cx="6" cy="18" r="3"></circle>
      <path d="M6 6c6.627 0 12 5.373 12 12"></path>
      <path d="M6 12c3.314 0 6 2.686 6 6"></path>
    </svg>
  `;

  const printButton = menuBar.querySelector(".print-btn, #print-button");
  if (printButton && printButton.parentNode === menuBar) {
    printButton.before(rssLink);
  } else {
    menuBar.appendChild(rssLink);
  }
});
