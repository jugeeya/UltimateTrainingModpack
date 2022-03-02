var isNx = (typeof window.nx !== 'undefined')
var prevQuestionMsg = null
var prevFocusedElm = null

if (isNx) {
    window.nx.footer.setAssign('B', '', goBackHook, {
        se: ''
    })
    window.nx.footer.setAssign('X', '', resetSubmenu, {
        se: ''
    })
    window.nx.footer.setAssign('L', '', resetAllSubmenus, {
        se: ''
    })
    window.nx.footer.setAssign('R', '', toggleSaveDefaults, {
        se: ''
    })
} else {
    document.getElementById("body").addEventListener('keypress', (event) => {
        switch (event.key) {
            case "b":
                console.log("b");
                goBackHook();
                break;
            case "x":
                console.log("x");
                resetSubmenu();
                break;
            case "l":
                console.log("l");
                resetAllSubmenus();
                break;
            case "r":
                console.log("r");
                toggleSaveDefaults();
                break;
        }
    });
}

window.onload = setSettings;

function isTextNode(node) {
    return node.nodeType == Node.TEXT_NODE
}

function closestClass(elem, class_) {
    // Returns the closest anscestor (including self) with the given class
    if (!elem) {
        // Reached the end of the DOM
        return null
    } else if (elem.classList.contains(class_)) {
        // Found it
        return elem
    } else {
        // Didn't find it, go up a level
        return closestClass(elem.parentElement, class_);
    }
}

function getElementByXpath(path) {
    return document.evaluate(path, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;
}

function focusQA(e) {
    playSound("SeSelectUncheck");
    prevFocusedElm = e;
    e.classList.add("is-focused")
}

function defocusQA(e) {
    if (prevFocusedElm) {
        prevFocusedElm.classList.remove('is-focused')

    }
    if (prevQuestionMsg) {
        prevQuestionMsg.remove()
        prevQuestionMsg = null
    }
}

function toggleAnswer(e) {
    playSound("SeToggleBtnOn")
    e.classList.toggle("is-opened")

    // Toggle visibility of child answers
    e.childNodes.forEach((child) => {
        if (!isTextNode(child) && child.classList.contains("answer-border-outer")) {
            child.classList.toggle("is-hidden");
        }
    });

    // Toggle visibility of sibling answers
    var sibling = e.nextSibling
    if (sibling.classList.contains("answer-border-outer")) {
        sibling.classList.toggle("is-hidden")
    }
}

function playSound(label) {
    // Valid labels:
    // SeToggleBtnFocus
    // SeToggleBtnOn
    // SeToggleBtnOff
    // SeCheckboxFocus
    // SeCheckboxOn
    // SeCheckboxOff
    // SeRadioBtnFocus
    // SeRadioBtnOn
    // SeSelectCheck
    // SeSelectUncheck
    // SeBtnDecide
    // SeTouchUnfocus
    // SeBtnFocus
    // SeKeyError
    // SeDialogOpen
    // SeWebZoomOut
    // SeWebZoomIn
    // SeWebNaviFocus
    // SeWebPointerFocus
    // SeFooterFocus
    // SeFooterDecideBack
    // SeFooterDecideFinish
    // SeWebChangeCursorPointer
    // SeWebTouchFocus
    // SeWebLinkDecide
    // SeWebTextboxStartEdit
    // SeWebButtonDecide
    // SeWebRadioBtnOn
    // SeWebCheckboxUncheck
    // SeWebCheckboxCheck
    // SeWebMenuListOpen
    if (isNx) {
        window.nx.playSystemSe(label);
    } else {
        console.log("Sound Effect: " + label);
    }
}

function goBackHook() {
    // If any submenus are open, close them
    // Otherwise if all submenus are closed, exit the menu and return to the game
    if (document.querySelectorAll(".qa.is-opened").length == 0) {
        // If all submenus are closed, exit and return through localhost
        playSound("SeFooterDecideBack");

        var url = "http://localhost/"

        var settings = [];

        // Collect settings for toggles
        
        document.querySelectorAll("ul.l-grid").forEach((toggle) => {
            var section = toggle.id;
            var val = "";

            toggle.childNodes.forEach((child) => {
                if (!isTextNode(child) && child.querySelectorAll(".is-appear").length) {
                    val += child.getAttribute("val") + ",";
                };
            });

            settings.push({
                name: section,
                value: val
            });
        });

        // Collect settings for OnOffs
        document.querySelectorAll("div.onoff").forEach((onoff) => {
            var section = onoff.id;
            var val = "";
            if (onoff.querySelectorAll(".is-appear").length) {
                val = "1"
            } else {
                val = "0"
            }
            settings.push({
                name: section,
                value: val
            });
        });

        url += "?" + decodeURIComponent($.param(settings));
        if (document.getElementById("saveDefaults").checked) {
            url += "&save_defaults=1";
        }
        
        if (isNx) {
            location.href = url;
            window.nx.endApplet();
        } else {
            console.log(url);
        }
    } else {
        // Close any open submenus
        document.querySelectorAll(".qa.is-opened").forEach((submenu) => { toggleAnswer(submenu); });
    }
}

function clickToggle(e) {
    playSound("SeCheckboxOn")
    var toggleOptions = e.querySelector(".toggle");
    if (e.querySelector(".is-single-option")) { // Single-option submenu
        // Deselect all submenu options
        closestClass(e, "l-qa").querySelector(".toggle").classList.remove("is-appear");
        closestClass(e, "l-qa").querySelector(".toggle").classList.add("is-hidden");
        // Then set the current one as the active setting
        toggleOptions.classList.add("is-appear");
        toggleOptions.classList.remove("is-hidden");
    } else { // Multi-option submenu
        toggleOptions.classList.toggle("is-appear");
        toggleOptions.classList.toggle("is-hidden");
    }
}

function getParams(url) {
    var regex = /[?&]([^=#]+)=([^&#]*)/g,
        params = {},
        match;
    while (match = regex.exec(url)) {
        params[match[1]] = match[2];
    }
    return params;
}

function setSettings() {
    // Get settings from the URL GET parameters
    const settings = getParams(document.URL);

    // Set Toggles
    document.querySelectorAll("ul.l-grid").forEach((toggle) => {
        var section = toggle.id;
        var section_setting = decodeURIComponent(settings[section]);

        toggle.querySelectorAll("li").forEach((child) => {
            var e = child.querySelector("img.toggle");
            if (section_setting.split(",").includes(child.getAttribute("val"))) {
                e.classList.add("is-appear");
                e.classList.remove("is-hidden");
            } else {
                e.classList.remove("is-appear");
                e.classList.add("is-hidden");
            };
        });
    });

    // Set OnOffs
    document.querySelectorAll("div.onoff").forEach((onOff) => {
        var section = onOff.id;
        var section_setting = decodeURIComponent(settings[section]);
        var e = onOff.querySelector("img.toggle");
        if (section_setting == "1") {
            e.classList.add("is-appear");
            e.classList.remove("is-hidden");
        } else {
            e.classList.remove("is-appear");
            e.classList.add("is-hidden");
        };
    });
}

function resetSubmenu() {
    // Resets any open or focused submenus to the default values
    playSound("SeToggleBtnOff");
    document.querySelectorAll("[default*='is-appear']").forEach((item) => {
        if (isSubmenuFocused(item)) {
            item.classList.add("is-appear");
            item.classList.remove("is-hidden");
        }
    });

    document.querySelectorAll("[default*='is-hidden']").forEach((item) => {
        if (isSubmenuFocused(item)) {
            item.classList.remove("is-appear");
            item.classList.add("is-hidden");
        }
    });
}

function isSubmenuFocused(elem) {
    // Return true if the element is in a submenu which is either focused or opened
    return (
        closestClass(elem, "l-qa").querySelectorAll(".is-opened, .is-focused").length
        || closestClass(elem, "is-focused")
    )
}

function resetAllSubmenus() {
    // Resets all submenus to the default values
    if (confirm("Are you sure that you want to reset all menu settings to the default?")) {
        playSound("SeToggleBtnOff");
        document.querySelectorAll("[default*='is-appear']").forEach((item) => {
            item.classList.add("is-appear");
            item.classList.remove("is-hidden");
        });
    
        document.querySelectorAll("[default*='is-hidden']").forEach((item) => {
            item.classList.remove("is-appear");
            item.classList.add("is-hidden");
        });
    }
}

function setHelpText(text) {
    // Modify the help text in the footer
    document.getElementById("help-text").innerText = text;
}

function toggleSaveDefaults() {
    // Change the status of the Save Defaults checkbox
    playSound("SeCheckboxOn");
    var saveDefaultsCheckbox = document.getElementById("saveDefaults");
    saveDefaultsCheckbox.checked = !saveDefaultsCheckbox.checked;
}