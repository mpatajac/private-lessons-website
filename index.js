function setup() {
	// "animate-on-scroll" elements - pre-fetched to avoid refetching on every scroll event
	const aosSingleElems = document.getElementsByClassName("aos");
	const aosCollectionElems = document.getElementsByClassName("aos-collection");

	window.onscroll = () => {
		handleNavbarUnderline();
		applyAOS(aosSingleElems, aosCollectionElems);
	}

	// check on page load if any elements need to be shown
	applyAOS(aosSingleElems, aosCollectionElems);
}

// --------------------------------------------------


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


// --------------------------------------------------

// Used to delay element display (wait until it has scrolled enough)
const checkDelta = 200;

// Checks if element satisfies all criteria (namely, vertical position) in order to be displayed
function isElemVisible(elem) {
	// `elem.getBoundingClientRect().top` gets elements distance from the top of the viewport
	// so we need to check if the top has reached viewport bottom (- delta)
	const viewportBottom = window.screen.availHeight - checkDelta;
	const elementTop = elem.getBoundingClientRect().top;

	return viewportBottom > elementTop;
};


// Apply logic so specified elements can perform animation when they are scrolled to
function applyAOS(singleElems, elemCollections) {
	// singular elements
	for (const elem of singleElems) {
		if (isElemVisible(elem)) {
			elem.classList.add("animated");
		}
	}

	// element collections
	for (const parentElem of elemCollections) {
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

// --------------------------------------------------