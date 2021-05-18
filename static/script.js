function moveCaretToEnd(el) {
  if (typeof el.selectionStart == "number") {
    el.selectionStart = el.selectionEnd = el.value.length;
  } else if (typeof el.createTextRange != "undefined") {
    el.focus();
    var range = el.createTextRange();
    range.collapse(false);
    range.select();
  }
}

function moveCaretToStart(el) {
  if (typeof el.selectionStart == "number") {
    el.selectionStart = el.selectionEnd = 0;
  } else if (typeof el.createTextRange != "undefined") {
    el.focus();
    var range = el.createTextRange();
    range.collapse(false);
    range.select();
  }
}

function edit() {
  const editElement = document.getElementById("edit");
  if (editElement) {
    // sometimes the page might not be editable
    editElement.checked = true;
  }
}

function search() {
  window.location.pathname = "/search";
}

function jumpNew() {
  window.location.pathname = "/new";
}

const textarea = document.getElementById("body");
if (textarea) {
  textarea.onkeydown = function (e) {
    switch (e.key) {
      case "Home":
        moveCaretToStart(textarea);
        break;
      case "End":
        moveCaretToEnd(textarea);
        break;
      default:
        break;
    }
  };
}
document.onkeydown = function (e) {
  if (e.target !== document.body) return;
  switch (e.key) {
    case "e":
      edit();
      break;
    case "/":
      search();
      break;
    case "n":
      jumpNew();
      break;
    default:
      break;
  }
};

function replaceLinkTo() {
  const linkTo = document.getElementById("linkto");
  if (linkTo) return;
  linkTo.href = `/new?linkto=${window.location.pathname.slice(1)}`;
}

replaceLinkTo();
