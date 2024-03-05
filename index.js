function setup() {
	// TODO?: collect all `aos` elements here, pass them to `applyAOS` to prevent re-fetching on every scroll?
	window.onscroll = () => {
		handleNavbarUnderline();
		applyAOS();
	}

	// check on page load if any elements need to be shown
	applyAOS();
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


// Apply logic so specified elements can perform animation when they are scrolled to
function applyAOS() {
	// used to delay element display (wait until it has scrolled enough)
	// const checkDelta = window.screen.availHeight / 4;
	const checkDelta = 200;
	const isElemVisible = (elem) => {
		// `elem.getBoundingClientRect().top` gets elements distance from the top of the viewport
		// so we need to check if the top has reached viewport bottom (- delta)
		const viewportBottom = window.screen.availHeight - checkDelta;
		const elementTop = elem.getBoundingClientRect().top;

		return viewportBottom > elementTop;
	};

	// singular elements
	const aosElems = document.getElementsByClassName("aos");
	for (const elem of aosElems) {
		if (isElemVisible(elem)) {
			elem.classList.add("animated");
		}
	}

	// element collections
	const aosCollectionElems = document.getElementsByClassName("aos-collection");
	for (const parentElem of aosCollectionElems) {
		let childElems = parentElem.children;

		for (let i = 0; i < childElems.length; ++i) {
			const elem = childElems[i];

			if (isElemVisible(elem)) {
				// add index alongside class to delay animation
				elem.classList.add("animated", `animated-stagger-${i}`);
			}
		}
	}
}