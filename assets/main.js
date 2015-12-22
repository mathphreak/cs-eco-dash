function update() {
  $.getJSON('/data.json').done(function(data) {
    // clearly the server is up
    $("#server-down").hide();

    // show some data
    $("#money").text(data.money)
    $("#gsi-installed").text(data.gsi.installed)
    $("#gsi-target").text(data.gsi.target)
    $("#gamemode").text(data.gamemode);
    $("#map").text(data.map);
    $("#recommendations").html("");
    if (data.recommendations) {
      data.recommendations.forEach(function(rec) {
        $("#recommendations").append($("<li>" + rec + "</li>"));
      })
    }

    // hide some stuff
    var showInstallConfig = data.gsi.installed !== data.gsi.target;
    var showFail = !data.up;
    var inGame = data.in_game;
    $("#gsi-install-config").toggle(showInstallConfig);
    $("#csgo-down").toggle(showFail);
    $("#fail-icons").toggle(showFail);
    $("#out-of-game").toggle(!inGame);
    $("#in-game").toggle(inGame);
    $("#game-status").toggle(inGame);

    setTimeout(update, 100);
  }).fail(function() {
    $("#fail-icons").show();
    $("#server-down").show();
    $("#csgo-down").hide();
    $("#out-of-game").show();
    $("#in-game").hide();

    setTimeout(update, 5000);
  })
}

$(function() {
  update()
  setTimeout(update, 100)
})
