var isNx = (typeof window.nx !== 'undefined')
var $prevQuestionMsg = null
var $prevFocusedElm = null
var scrollTimeoutId = null
var OFFSET_Y = 85

function focusQA(e) {
    playSound("SeSelectUncheck");
    $prevFocusedElm = e;
    e.classList.add("is-focused")
}

function defocusQA(e) {
    if ($prevFocusedElm) {
        $prevFocusedElm.classList.remove('is-focused')

    }
    if ($prevQuestionMsg) {
        $prevQuestionMsg.remove()
        $prevQuestionMsg = null
    }
}

function toggleAnswer(e) {
    playSound("SeToggleBtnOn")
    e.classList.toggle("is-opened")

    // Toggle visibility of child answers
    var children = e.children
    for (var i = 0; i < children.length; i++) {
        var child = children[i]
        if (child.classList.contains("answer-border-outer")) {
            child.classList.toggle("is-hidden")
        }
    }

    // Toggle visibility of sibling answers
    var sibling = e.nextSibling
    if (sibling.classList.contains("answer-border-outer")) {
        sibling.classList.toggle("is-hidden")
    }

    if (e.classList.contains("is-opened")) {
        scrollQA(e.id)
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

function scrollQA(qaId, endFunc) {
    isScrolling = true
    isStopCursorAnim = true
    clearTimeout(scrollTimeoutId)
    scrollTimeoutId = null

    var scrollTop = getScrollTop(qaId)

    var windowScrollTop = $(window).scrollTop()
    if (scrollTop === windowScrollTop) {
        isScrolling = false
        isStopCursorAnim = false
        return
    }
}

function getScrollTop(qaId) {
    var $qaElm = $('#' + qaId)
    var qaPos = $qaElm.offset().top
    return qaPos - OFFSET_Y
}

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
}
function goBackHook() {
    // If any submenus are open, close them
    // Otherwise if all submenus are closed, exit the menu and return to the game
    if ($(".qa.is-opened").length == 0) {
        // If all submenus are closed, exit and return through localhost
        playSound("SeFooterDecideBack");

        var url = "http://localhost/"

        var settings = [];

        // Collect settings for toggles
        $("ul.l-grid").each(function () {
            var section = this.id;
            var val = "";

            var children = this.children;
            for (var i = 0; i < children.length; i++) {
                var child = children[i];
                if ($(child).find(".is-appear").length) {
                    val += child.getAttribute("val") + ",";
                };
            }

            settings.push({
                name: section,
                value: val
            });
        });

        // Collect settings for OnOffs
        $("div.onoff").each(function () {
            var section = this.id;
            var val = "";
            if ($(this).find(".is-appear").length) {
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
        if ($("#saveDefaults").prop("checked")) {
            url += "&save_defaults=1";
        }
        console.log(url);
        location.href = url;

        window.nx.endApplet();
    } else {
        // Close any open submenus
        $(".qa.is-opened").each(function () { toggleAnswer(this); });
    }
}

function clickToggle(e) {
    playSound("SeCheckboxOn")
    var toggleOptions = $(e).find(".toggle");
    if ($(e).find(".is-single-option").length) { // Single-option submenu
        // Deselect all submenu options
        $(e).closest(".l-qa").find(".toggle").removeClass("is-appear");
        $(e).closest(".l-qa").find(".toggle").addClass("is-hidden");
        // Then set the current one as the active setting
        toggleOptions.addClass("is-appear");
        toggleOptions.removeClass("is-hidden");
    } else { // Multi-option submenu
        toggleOptions.toggleClass("is-appear");
        toggleOptions.toggleClass("is-hidden");
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
    $("ul.l-grid").each(function () {
        var section = this.id;
        var section_setting = decodeURIComponent(settings[section]);

        var children = $(this).children("li");
        for (var i = 0; i < children.length; i++) {
            var child = children[i];
            var e = $(child).find("img.toggle");
            if (section_setting.split(",").includes(child.getAttribute("val"))) {
                e.addClass("is-appear");
                e.removeClass("is-hidden");
            } else {
                e.removeClass("is-appear");
                e.addClass("is-hidden");
            };
        }
    });

    // Set OnOffs
    $("div.onoff").each(function () {
        var section = this.id;
        var section_setting = decodeURIComponent(settings[section]);
        var e = $(this).find("img.toggle");
        if (section_setting == "1") {
            e.addClass("is-appear");
            e.removeClass("is-hidden");
        } else {
            e.removeClass("is-appear");
            e.addClass("is-hidden");
        };
    });
}

function resetSubmenu() {
    // Resets any open or focused submenus to the default values
    playSound("SeToggleBtnOff");
    $("[default*='is-appear']").each(function () {
        if (isSubmenuFocused(this)) {
            $(this).addClass("is-appear");
            $(this).removeClass("is-hidden");
        }
    });
    $("[default*='is-hidden']").each(function () {
        if (isSubmenuFocused(this)) {
            $(this).removeClass("is-appear");
            $(this).addClass("is-hidden");
        }
    });
}

function isSubmenuFocused(elem) {
    // Return true if the element is in a submenu which is either focused or opened
    return (
        $(elem).closest(".l-qa").children(".is-opened, .is-focused").length
        || $(elem).closest(".is-focused").length
    )
}

function resetAllSubmenus() {
    // Resets all submenus to the default values
    if (confirm("Are you sure that you want to reset all menu settings to the default?")) {
        playSound("SeToggleBtnOff");
        $("[default*='is-appear']").addClass("is-appear");
        $("[default*='is-appear']").removeClass("is-hidden");

        $("[default*='is-hidden']").removeClass("is-appear");
        $("[default*='is-hidden']").addClass("is-hidden");
    }
}

function setHelpText(text) {
    // Modify the help text in the footer
    $("#help-text").text(text);
}

function toggleSaveDefaults() {
    // Change the status of the Save Defaults checkbox
    playSound("SeCheckboxOn");
    var saveDefaultsCheckbox = $("#saveDefaults");
    saveDefaultsCheckbox.prop(
        "checked",
        !saveDefaultsCheckbox.prop("checked")
    );
}