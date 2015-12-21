function update() {
  $.getJSON('/data.json', function(data, status, xhr) {
    $("#money").text(data.money)
    $("#gsi-installed").text(data.gsi.installed)
    $("#gsi-target").text(data.gsi.target)
    if (data.gsi.installed === data.gsi.target) {
      $("#gsi-install-config").hide();
    } else {
      $("#gsi-install-config").show();
    }
    $("#recommendations").html("");
    data.recommendations.forEach(function(rec) {
      $("#recommendations").append($("<li>" + rec + "</li>"));
    })
  })
}

$(function() {
  update()
  setInterval(update, 1000)
})
