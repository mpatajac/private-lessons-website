window.onscroll = () => {
	handleNavbarUnderline();
}



// Add or remove navbar shadow depending on whether page is scrolled or not
function handleNavbarUnderline() {
	// NOTE: one (and only one) navbar should exist, so indexing should be fine
	const navbar = document.getElementsByClassName("navbar")[0];

	// window.scrollY > 0 => page was scrolled from initial position
	// shadow should be present when page is scrolled
	if (window.scrollY > 0) {
		navbar.classList.add("has-shadow");
	} else {
		navbar.classList.remove("has-shadow");
	}
}


// Apply (smooth) scrolling back to the top of the page
function scrollToTop() {
	document.documentElement.scrollTop = 0;
}