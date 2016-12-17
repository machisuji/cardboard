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
    },
    updateCard: function(a) {
      var self = this;
      var card = jQuery(a).closest(".card");

      jQuery.ajax({
        url: "/update_card",
        type: "POST",
        dataType: "json",
        data : card.find("form").serialize(),
        success : function(result) {
          location.reload();
        },
        error: function(res) {
          alert("Error: " + res.responseJSON.message);
        }
      });
    },
    editCard: function(a) {
      var card = jQuery(a).closest(".card");

      this.toggleUpdateForm(card);

      if (card.is(":visible")) {
        card.find("textarea").focus();
      }
    },
    cancelEdit: function(a) {
      var card = jQuery(a).closest(".card");

      this.toggleUpdateForm(card);
    },
    toggleUpdateForm: function(card) {
      card.find(".content").toggle();
      card.find(".form").toggle();
    }
  }

  window.Cardboard = new CardboardApp();
})(window, jQuery);
