function update() {
  $.getJSON('/data.json').done(function(data) {
    $("#money").text(data.money)
    $("#gsi-installed").text(data.gsi.installed)
    $("#gsi-target").text(data.gsi.target)
    if (data.gsi.installed === data.gsi.target) {
      $("#gsi-install-config").hide();
    } else {
      $("#gsi-install-config").show();
    }
    $("#recommendations").html("");
    if (data.recommendations) {
      data.recommendations.forEach(function(rec) {
        $("#recommendations").append($("<li>" + rec + "</li>"));
      })
    }
    $("#server-down").hide();
    if (data.up) {
      $("#csgo-down").hide();
      $("#fail-icons").hide();
    } else {
      $("#csgo-down").show();
      $("#fail-icons").show();
    }
    if (data.in_game) {
      $("#out-of-game").hide();
      $("#in-game").show();
    } else {
      $("#out-of-game").show();
      $("#in-game").hide();
    }
  }).fail(function() {
    $("#fail-icons").show();
    $("#server-down").show();
    $("#csgo-down").hide();
    $("#out-of-game").show();
    $("#in-game").hide();
  })
}

$(function() {
  update()
  setInterval(update, 1000)
})
