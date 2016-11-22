(function (window, $) {
  function CardboardApp() {

  };

  CardboardApp.prototype = {
    constructor: CardboardApp,
    allowDrop: function(event) {
      if ($(event.target).hasClass("board")) {
        event.preventDefault();
      }
    },
    drag: function(event) {
      event.dataTransfer.setData("text", event.target.id);
    },
    drop: function(event) {
      var id = event.dataTransfer.getData("text");

      event.preventDefault();
      event.target.appendChild(document.getElementById(id));
    }
  }

  window.Cardboard = new CardboardApp();
})(window, jQuery);
