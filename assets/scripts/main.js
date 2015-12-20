function update() {
  $.getJSON('/data.json', function(data, status, xhr) {
    $("#money").text(data.money)
    $("#gsi-installed").text(data.gsi.installed)
    $("#gsi-target").text(data.gsi.target)
  })
}

$(function() {
  update()
  setInterval(update, 1000)
})
