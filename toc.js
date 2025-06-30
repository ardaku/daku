// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="glossary.html"><strong aria-hidden="true">2.</strong> Glossary</a></li><li class="chapter-item expanded "><a href="primitives.html"><strong aria-hidden="true">3.</strong> Primitives</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="list.html"><strong aria-hidden="true">3.1.</strong> List</a></li><li class="chapter-item expanded "><a href="buffer.html"><strong aria-hidden="true">3.2.</strong> Buffer</a></li><li class="chapter-item expanded "><a href="command.html"><strong aria-hidden="true">3.3.</strong> Command</a></li></ol></li><li class="chapter-item expanded "><a href="runtime.html"><strong aria-hidden="true">4.</strong> Runtime</a></li><li class="chapter-item expanded "><a href="channels.html"><strong aria-hidden="true">5.</strong> Channels</a></li><li class="chapter-item expanded "><a href="daku.html"><strong aria-hidden="true">6.</strong> Daku Custom Section</a></li><li class="chapter-item expanded "><a href="types.html"><strong aria-hidden="true">7.</strong> Portal Types</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="text.html"><strong aria-hidden="true">7.1.</strong> Text</a></li><li class="chapter-item expanded "><a href="vector.html"><strong aria-hidden="true">7.2.</strong> Vector</a></li><li class="chapter-item expanded "><a href="positions.html"><strong aria-hidden="true">7.3.</strong> Positions</a></li><li class="chapter-item expanded "><a href="audio.html"><strong aria-hidden="true">7.4.</strong> Audio</a></li><li class="chapter-item expanded "><a href="dimensions.html"><strong aria-hidden="true">7.5.</strong> Dimensions</a></li><li class="chapter-item expanded "><a href="raster.html"><strong aria-hidden="true">7.6.</strong> Raster 🧪</a></li><li class="chapter-item expanded "><a href="timestamp.html"><strong aria-hidden="true">7.7.</strong> Timestamp 🧪</a></li><li class="chapter-item expanded "><a href="date.html"><strong aria-hidden="true">7.8.</strong> Date 🧪</a></li><li class="chapter-item expanded "><a href="time.html"><strong aria-hidden="true">7.9.</strong> Time 🧪</a></li><li class="chapter-item expanded "><a href="datetime.html"><strong aria-hidden="true">7.10.</strong> DateTime 🧪</a></li><li class="chapter-item expanded "><a href="lang.html"><strong aria-hidden="true">7.11.</strong> Lang 🧪</a></li><li class="chapter-item expanded "><a href="region.html"><strong aria-hidden="true">7.12.</strong> Region 🧪</a></li><li class="chapter-item expanded "><a href="langregion.html"><strong aria-hidden="true">7.13.</strong> LangRegion 🧪</a></li><li class="chapter-item expanded "><a href="leapsecond.html"><strong aria-hidden="true">7.14.</strong> LeapSecond 🧪</a></li><li class="chapter-item expanded "><a href="timedesignation.html"><strong aria-hidden="true">7.15.</strong> TimeDesignation 🧪</a></li><li class="chapter-item expanded "><a href="timeadjustment.html"><strong aria-hidden="true">7.16.</strong> TimeAdjustment 🧪</a></li><li class="chapter-item expanded "><a href="timezone.html"><strong aria-hidden="true">7.17.</strong> TimeZone 🧪</a></li></ol></li><li class="chapter-item expanded "><a href="portals.html"><strong aria-hidden="true">8.</strong> Portals</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="log.html"><strong aria-hidden="true">8.1.</strong> 0x00 - Log</a></li><li class="chapter-item expanded "><a href="prompt.html"><strong aria-hidden="true">8.2.</strong> 0x01 - Prompt</a></li><li class="chapter-item expanded "><a href="fetch.html"><strong aria-hidden="true">8.3.</strong> 0x02 - Fetch 🧪</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="host.html"><strong aria-hidden="true">8.3.1.</strong> Device/Host 🧪</a></li></ol></li><li class="chapter-item expanded "><a href="serve.html"><strong aria-hidden="true">8.4.</strong> 0x03 - Serve 🧪</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="client.html"><strong aria-hidden="true">8.4.1.</strong> Device/Client 🧪</a></li></ol></li><li class="chapter-item expanded "><a href="speakers.html"><strong aria-hidden="true">8.5.</strong> 0x04 - Speakers 🧪</a></li><li class="chapter-item expanded "><a href="microphone.html"><strong aria-hidden="true">8.6.</strong> 0x05 - Microphone 🧪</a></li><li class="chapter-item expanded "><a href="screen.html"><strong aria-hidden="true">8.7.</strong> 0x06 - Screen 🧪</a></li><li class="chapter-item expanded "><a href="camera.html"><strong aria-hidden="true">8.8.</strong> 0x07 - Camera 🧪</a></li><li class="chapter-item expanded "><a href="window.html"><strong aria-hidden="true">8.9.</strong> 0x08 - Window 🧪</a></li><li class="chapter-item expanded "><a href="spawn.html"><strong aria-hidden="true">8.10.</strong> 0x09 - Spawn 🧪</a></li><li class="chapter-item expanded "><a href="user.html"><strong aria-hidden="true">8.11.</strong> 0x0A - User 🧪</a></li><li class="chapter-item expanded "><a href="preferences.html"><strong aria-hidden="true">8.12.</strong> 0x0B - Preferences 🧪</a></li><li class="chapter-item expanded "><a href="system.html"><strong aria-hidden="true">8.13.</strong> 0x0C - System 🧪</a></li><li class="chapter-item expanded "><a href="about.html"><strong aria-hidden="true">8.14.</strong> 0x0D - About 🧪</a></li><li class="chapter-item expanded "><a href="file.html"><strong aria-hidden="true">8.15.</strong> 0x0E - File 🧪</a></li><li class="chapter-item expanded "><a href="hid.html"><strong aria-hidden="true">8.16.</strong> 0x0F - Hid 🧪</a></li><li class="chapter-item expanded "><a href="timer.html"><strong aria-hidden="true">8.17.</strong> 0x10 - Timer 🧪</a></li><li class="chapter-item expanded "><a href="clock.html"><strong aria-hidden="true">8.18.</strong> 0x11 - Clock 🧪</a></li><li class="chapter-item expanded "><a href="gpu.html"><strong aria-hidden="true">8.19.</strong> 0x12 - Gpu 🧪</a></li><li class="chapter-item expanded "><a href="location.html"><strong aria-hidden="true">8.20.</strong> 0x13 - Location 🧪</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
