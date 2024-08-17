/**
 * *********
 * VARIABLES
 * *********
 */

// Represents the last scroll's timestamp
let lastScrollTime = 0;
// Set a cooldown period to prevent too frequent scrolling
const scrollCooldown = 800;
// Represents if about us section has ran or not
let hasAboutUsRan = false;
// Elements
const headerMenuItems = document.querySelectorAll(".header-menu-item");
const triangleElement = document.querySelector(".header-triangle");

/**
 * ******
 * EVENTS
 * ******
 */

// Change section by click
headerMenuItems.forEach((item) => {
  item.addEventListener("click", function () {
    handleChangeSectionByClick(this);
  });
});

// Change section by scroll
document.addEventListener(
  "wheel",
  function (event) {
    handleChangeSectionByScroll(event);
  },
  { passive: false }
);

document.addEventListener(
  "touchend",
  function (event) {
    const position = window.scrollY;
    const deviceHeight = window.screen.height;
    targetSectionIndex = Math.floor((position + 50) / deviceHeight);
    const sections = Array.from(headerMenuItems);
    deactivateAllMenuItems();
    activateMenuItem(sections[targetSectionIndex]);
  },
  { passive: false }
);

let currentSection = "header";
document.addEventListener("DOMContentLoaded", function () {
  handleDOMLoaded();
});

// Accordion
class Accordion {
  static activeAccordion = null;

  constructor(el) {
    this.el = el;
    this.summary = el.querySelector("summary");
    this.content = el.querySelector(".accordion-content");
    this.expandIcon = this.summary.querySelector(".accordion-icon");
    this.animation = null;
    this.isClosing = false;
    this.isExpanding = false;
    this.summary.addEventListener("click", (e) => this.onClick(e));
  }

  onClick(e) {
    e.preventDefault();
    this.el.style.overflow = "hidden";

    if (Accordion.activeAccordion && Accordion.activeAccordion !== this) {
      Accordion.activeAccordion.shrink();
    }

    if (this.isClosing || !this.el.open) {
      this.open();
    } else if (this.isExpanding || this.el.open) {
      this.shrink();
    }
  }

  shrink() {
    this.isClosing = true;

    const startHeight = `${this.el.offsetHeight}px`;
    const endHeight = `${this.summary.offsetHeight}px`;

    if (this.animation) {
      this.animation.cancel();
    }

    this.animation = this.el.animate(
      { height: [startHeight, endHeight] },
      { duration: 400, easing: "ease-out" }
    );
    this.animation.onfinish = () => this.onAnimationFinish(false);
    this.animation.oncancel = () => (this.isClosing = false);
  }

  open() {
    this.el.style.height = `${this.el.offsetHeight}px`;
    this.el.open = true;
    window.requestAnimationFrame(() => this.expand());
  }

  expand() {
    this.isExpanding = true;

    const startHeight = `${this.el.offsetHeight}px`;
    const endHeight = `${
      this.summary.offsetHeight + this.content.offsetHeight
    }px`;

    if (this.animation) {
      this.animation.cancel();
    }

    this.animation = this.el.animate(
      { height: [startHeight, endHeight] },
      { duration: 350, easing: "ease-out" }
    );

    this.animation.onfinish = () => this.onAnimationFinish(true);
    this.animation.oncancel = () => (this.isExpanding = false);
  }

  onAnimationFinish(open) {
    this.el.open = open;
    this.animation = null;
    this.isClosing = false;
    this.isExpanding = false;
    this.el.style.height = this.el.style.overflow = "";
    if (open) {
      Accordion.activeAccordion = this;
    } else if (Accordion.activeAccordion === this) {
      Accordion.activeAccordion = null;
    }
  }
}

document.querySelectorAll("details").forEach((el) => {
  new Accordion(el);
});

/**
 * ********
 * HANDLERS
 * ********
 */

function handleChangeSectionByClick(targetSection) {
  if (!targetSection) {
    return;
  }

  deactivateAllMenuItems();
  activateMenuItem(targetSection);
  scrollToSection(targetSection.getAttribute("sectionId"));
}

function handleChangeSectionByScroll(event) {
  console.log("event By Scroll", event);
  if (event.cancelable) {
    event.preventDefault();
  }

  if (!canChangeSectionByScroll()) {
    return;
  }

  handleChangeSectionByClick(getTargetSectionByScroll(event));
}

function handleDOMLoaded() {
  const header = document.querySelector(".header");
  const howItWorks = document.querySelector(".how-it-works-content");
  const easySteps = document.querySelector(".easy-steps-content");
  const aboutUs = document.querySelector(".about-us-content");
  const footer = document.querySelector(".footer-content");

  // Create the observer, specifying the callback and options
  const observer = new IntersectionObserver(
    function (entries, observer) {
      // entries: Array of observed elements
      entries.forEach((entry, index) => {
        if (!entry.isIntersecting) {
          return;
        }

        triggerTheSectionLoad(entry);
      });
    },
    {
      root: null, // relative to the viewport
      rootMargin: "0px", // margin around the root
      threshold: 0.1, // percentage of target's visibility the observer's callback should execute
    }
  );

  // Attach the observer to the element
  observer.observe(header);
  observer.observe(howItWorks);
  observer.observe(aboutUs);
  observer.observe(easySteps);
  observer.observe(footer);

  scrollToSectionOnLoad();
  tiltElements();
}

function handleAboutUsLoad(entry) {
  moveTriangleToSection("how-it-works", "about-us", "third-part");

  if (hasAboutUsRan) {
    return;
  }

  hasAboutUsRan = true;
  createGridAnimationOnElement("about-us-image");
  createTextAnimation("about-us");
}

function handleHowItWorkLoad(entry) {
  createNetworkAnimation();
  createTextAnimation("how-it-works");
  moveTriangleToSection("header", "how-it-works", "second-part");
}

function handleHeaderLoad(entry) {
  // TODO: start rain animation here
  moveTriangleToSection("how-it-works", "header", "first-part");
}

function handleEasyStepsLoad(entry) {
  moveTriangleToSection("about-us", "easy-steps", "fourth-part");
  const cardsClassNames = ["download", "signup", "rewards"];

  cardsClassNames.forEach((className) => {
    const aboutUsTextDownload = document.querySelector(
      `.easy-steps-content-card-${className}`
    );
    if (aboutUsTextDownload) {
      aboutUsTextDownload.style.animation = `EasySteps${capitalizeFirstLetter(
        className
      )} 2.5s ease forwards`;
    }
  });
}

function handleFooterLoad(entry) {
  for (let index = 1; index <= 3; index++) {
    const footerBackgroundItem = document.querySelector(
      `.footer-background-cube-${index}`
    );
    if (footerBackgroundItem) {
      footerBackgroundItem.style.animation = `footerBackgroundCube${index} 1.5s ${
        index / 4
      }s ease forwards`;
    }
  }

  for (let index = 1; index <= 4; index++) {
    const footerBackgroundItem = document.querySelector(
      `.footer-background-line-${index}`
    );
    if (footerBackgroundItem) {
      footerBackgroundItem.style.animation = `footerBackgroundLine${index} 1.5s ${
        index / 4
      }s ease forwards`;
    }
  }
  currentSection = "footer";
}

/**
 * *****************
 * PRIVATE FUNCTIONS
 * *****************
 */

function triggerTheSectionLoad(entry) {
  if (entry.target.classList.contains("how-it-works-content")) {
    handleHowItWorkLoad(entry);
  } else if (entry.target.classList.contains("easy-steps-content")) {
    handleEasyStepsLoad(entry);
  } else if (entry.target.classList.contains("header")) {
    handleHeaderLoad(entry);
  } else if (entry.target.classList.contains("about-us-content")) {
    handleAboutUsLoad(entry);
  } else if (entry.target.classList.contains("footer-content")) {
    handleFooterLoad(entry);
  }
}

function scrollToSection(sectionId) {
  const section = document.getElementById(sectionId);
  if (section) {
    section.scrollIntoView({ behavior: "smooth", block: "center" });
  }
}

function activateMenuItem(element) {
  element.classList.add("header-menu-item-active");
  const cubeImage = element.querySelector(".header-menu-cube");
  if (cubeImage) {
    cubeImage.style.opacity = 1;
  }
}

function deactivateAllMenuItems() {
  headerMenuItems.forEach((otherItem) => {
    otherItem.classList.remove("header-menu-item-active");
    const cubeImage = otherItem.querySelector(".header-menu-cube");
    if (cubeImage) {
      cubeImage.style.opacity = 0;
    }
  });
}

function canChangeSectionByScroll() {
  const currentTime = new Date().getTime();
  const canChange = currentTime - lastScrollTime > scrollCooldown;
  if (canChange) {
    lastScrollTime = currentTime;
  }

  return canChange;
}

function getTargetSectionByScroll(event) {
  let delta;
  if (event.deltaY) {
    delta = event.deltaY;
  }

  const sections = Array.from(headerMenuItems);
  const currentSectionIndex = sections.findIndex((item) =>
    item.classList.contains("header-menu-item-active")
  );

  let targetSectionIndex;

  if (delta > 0 && currentSectionIndex < sections.length - 1) {
    // Scrolling down or swiping up
    targetSectionIndex = currentSectionIndex + 1;
  } else if (delta < 0 && currentSectionIndex > 0) {
    // Scrolling up or swiping down
    targetSectionIndex = currentSectionIndex - 1;
  } else {
    return; // If no valid movement direction, exit the function
  }

  return sections[targetSectionIndex];
}

function tiltElements() {
  const tiltable = document.querySelector(".tiltable");
  tiltable.addEventListener("mousemove", function (e) {
    const rect = this.getBoundingClientRect();
    const x = e.clientX - rect.left; // x position within the element
    const y = e.clientY - rect.top; // y position within the element

    const centerX = rect.width / 2;
    const centerY = rect.height / 2;

    const deltaX = x - centerX;
    const deltaY = y - centerY;

    // Strength of tilt: Higher values will create more tilt
    const strength = 20;

    // Calculate rotation values based on cursor position
    const rotateX = (deltaY / rect.height) * strength;
    const rotateY = (-deltaX / rect.width) * strength;

    // Apply the transformation
    this.style.transform = `perspective(500px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
  });

  tiltable.addEventListener("mouseleave", function () {
    // Reset the transformation when the mouse leaves
    this.style.transform = "none";
  });
}

function moveTriangleToSection(
  previousSection,
  currentSectionName,
  currentSectionPart
) {
  if (!triangleElement) {
    return;
  }

  triangleElement.classList.add(currentSectionPart);

  if (currentSection === previousSection) {
    triangleElement.style.animation = `triangle${kebabToPascalCase(
      currentSectionPart
    )} 1s forwards ease-out`;
  } else if (currentSection !== currentSectionName) {
    triangleElement.style.animation = `triangle${kebabToPascalCase(
      currentSectionPart
    )}Reverse 1s forwards ease-out`;
  }

  currentSection = currentSectionName;
}

function kebabToPascalCase(str) {
  return str
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

function capitalizeFirstLetter(string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

// Requires to have these div element with this class {NAME}-grid
function createGridAnimationOnElement(elementBaseClassName) {
  const grid = document.querySelector(`.${elementBaseClassName}-grid`);
  if (grid) {
    const numCubes = 100; // 10x10 grid
    grid.style.width = "100%";
    grid.style.height = "100%";
    // Create cubes
    for (let i = 0; i < numCubes; i++) {
      const cube = document.createElement("div");
      cube.classList.add(`${elementBaseClassName}-cube`);
      grid.appendChild(cube);
    }

    // Get all cubes and animate them randomly
    const cubes = document.querySelectorAll(`.${elementBaseClassName}-cube`);
    const shuffledIndices = [...Array(numCubes).keys()].sort(
      () => 0.5 - Math.random()
    );

    shuffledIndices.forEach((index, i) => {
      setTimeout(() => {
        cubes[index].style.opacity = 0;

        // Check if it's the last cube to disappear
        if (index === shuffledIndices[shuffledIndices.length - 1]) {
          setTimeout(() => {
            // Delete all cubes after the last one has faded
            cubes.forEach((cube) => grid.removeChild(cube));
            if (grid.parentNode) {
              grid.parentNode.removeChild(grid);
            }
          }, 250); // Wait a bit more to ensure the last opacity change is visually complete
        }
      }, i * 25);
    });
  }
}

function createTextAnimation(elementClassName) {
  const howItWorksText = document.querySelector(`.${elementClassName}-text`);
  if (howItWorksText) {
    howItWorksText.style.animation = `${kebabToPascalCase(
      elementClassName
    )}Text 1.5s ease forwards`;
  }
}

function createNetworkAnimation() {
  let delayTime = 0.5;
  for (let index = 1; index <= 20; index++) {
    const networkLineElement = document.querySelector(
      ".how-it-works-line-" + index
    );
    if (networkLineElement) {
      networkLineElement.style.animation = `visible 1s forwards ${delayTime}s`;
    }
    delayTime += 0.1;
  }
}

function scrollToSectionOnLoad() {
  const sectionsByOrder = [
    { section: "header", target: "header" },
    { section: "how-it-works", target: "how-it-works" },
    { section: "about-us", target: "about-us" },
    { section: "easy-steps", target: "easy-steps" },
    { section: "faq", target: "faq" },
    { section: "footer", target: "header" },
  ];

  const sectionHeight = window.innerHeight;
  const currentPosition = window.scrollY;
  const currentSectionIndex = Math.floor(currentPosition / sectionHeight);
  scrollToSection(sectionsByOrder[currentSectionIndex].target);
  const menuItems = document.querySelectorAll(".header-menu-item");
  deactivateAllMenuItems();
  activateMenuItem(menuItems[currentSectionIndex]);
}
