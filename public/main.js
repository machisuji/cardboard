(function(window, $) {
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

      jQuery.post({
        url: '/update_card',
        data: {
          card: {
            file_name: id,
            board: event.target.id
          }
        },
        dataType: "json",
        success: function() {
          event.preventDefault();
          event.target.appendChild(document.getElementById(id));
        },
        error: function(res) {
          alert("Error: " + res.responseJSON.message);
        }
      });
    }
  }

  window.Cardboard = new CardboardApp();
})(window, jQuery);
