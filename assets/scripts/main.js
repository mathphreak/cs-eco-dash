$(function() {
  setInterval(function() {
    $.getJSON('/data.json', function(data, status, xhr) {
      $("#money").text(data.money)
    })
  }, 100)
})
