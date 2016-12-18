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
      var card = jQuery(document.getElementById(id));
      var board = jQuery(event.target);

      if (card.parent().attr("id") !== board.attr("id")) {
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

      if (!this.isCardContentVisible(card)) {
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

      if (this.isCardContentVisible(card)) {
        card.attr("draggable", "true");
      } else {
        card.removeAttr("draggable");
      }
    },
    isCardContentVisible: function(card) {
      return card.find(".content").is(":visible");
    },
    newCard: function(a) {
      var link = jQuery(a);
      var form = link.parent().find(".card.-new");

      link.hide();
      form.show();

      form.find("textarea").focus();
    },
    cancelCreate: function(a) {
      var card = jQuery(a).closest(".board");
      var newLink = card.find("a.new-card");
      var form = card.find(".card.-new");

      form.hide();
      newLink.show();
    },
    createCard: function(a) {
      var form = jQuery(a).closest("form");

      jQuery.ajax({
        url: "/create_card",
        type: "POST",
        dataType: "json",
        data : form.serialize(),
        success : function(result) {
          location.reload();
        },
        error: function(res) {
          alert("Error: " + res.responseJSON.message);
        }
      });
    }
  }

  window.Cardboard = new CardboardApp();

  jQuery(document).ready(function() {
    jQuery(".card .toolbar")
      .mouseenter(function() {
        jQuery(this).closest(".card")
          .attr("draggable", "true")
          .addClass("active");
      })
      .mouseleave(function() {
        jQuery(this).closest(".card")
          .attr("draggable", "false")
          .removeClass("active");
      });
  });
})(window, jQuery);
