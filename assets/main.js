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
  }).fail(function() {
    $("#server-down").show();
  })
}

$(function() {
  update()
  setInterval(update, 1000)
})
