function setup() {
	// navbar (underline toggle) - pre-fetched to avoid refetching on every scroll event
	const navbar = document.getElementById("navbar");

	// "animate-on-scroll" elements - pre-fetched to avoid refetching on every scroll event
	const aosSingleElems = document.getElementsByClassName("aos");
	const aosCollectionElems = document.getElementsByClassName("aos-collection");

	window.onscroll = () => {
		handleNavbarUnderline(navbar);
		applyAOS(aosSingleElems, aosCollectionElems);
	}

	handleNavbarUnderline(navbar);
	// check on page load if any elements need to be shown
	applyAOS(aosSingleElems, aosCollectionElems);
}

// --------------------------------------------------


// Add or remove navbar shadow depending on whether page is scrolled or not
function handleNavbarUnderline(navbar) {
	// window.scrollY > 0 => page was scrolled from initial position
	// shadow should be present when page is scrolled
	navbar.classList.toggle("has-shadow", window.scrollY > 0);
}


// Apply (smooth) scrolling back to the top of the page
function scrollToTop() {
	document.documentElement.scrollTop = 0;
}


// --------------------------------------------------

// Checks if element satisfies all criteria (namely, vertical position) in order to be displayed
function isElemVisible(elem) {
	// `elem.getBoundingClientRect().bottom` gets element's bottom's distance from the top of the viewport
	// so we need to check if the bottom has (roughly) reached viewport bottom (i.e. completely in viewport)
	/*
		TODO?: re-check this after all the content is done to see if it behaves nicely 
			(e.g. maybe check `elem.getBoundingClientRect().top` instead of `.bottom` on smaller screens) 
	*/
	const viewportBottom = window.screen.availHeight;
	const elementBottom = elem.getBoundingClientRect().bottom;

	return viewportBottom > elementBottom;
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
				elem.classList.add("animated");
			}
		}
	}
}

// --------------------------------------------------