function setup() {
	// navbar (underline toggle) - pre-fetched to avoid refetching on every scroll event
	const navbar = document.getElementById("navbar");

	// "animate-on-scroll" elements - pre-fetched to avoid refetching on every scroll event
	const aosSingleElems = document.getElementsByClassName("aos");
	const aosCollectionElems = document.getElementsByClassName("aos-collection");

	const aosObserver = initAOSObserver();
	applyAOSObserver(aosObserver, aosSingleElems, aosCollectionElems);

	window.onscroll = () => {
		handleNavbarUnderline(navbar);
	}

	handleNavbarUnderline(navbar);
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

function initAOSObserver() {
	const intersectionCallback = (entries) => {
		for (const entry of entries) {
			if (entry.isIntersecting) {
				entry.target.classList.add("animated");
			}
		}
	};

	// on larger screens, wait until a major part of the element is visible
	// on smaller screens, only wait for a portion of the element to be visible
	const intersectionOptions = {
		threshold: window.visualViewport.width >= 768 ? 0.6 : 0.25
	};

	return new IntersectionObserver(intersectionCallback, intersectionOptions);
}

// Apply logic so specified elements can perform animation when they are scrolled to
function applyAOSObserver(observer, singleElems, elemCollections) {
	// singular elements
	for (const elem of singleElems) {
		observer.observe(elem);
	}

	// element collections
	for (const parentElem of elemCollections) {
		for (const elem of parentElem.children) {
			observer.observe(elem);
		}
	}
}

// --------------------------------------------------