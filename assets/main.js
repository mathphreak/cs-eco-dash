/* global $ */

function update () {
  $.getJSON('/data.json').done(function (data) {
    // clearly the server is up
    $('#server-down').hide()

    // show some data
    $('#money').text(data.money)
    $('#gsi-installed').text(data.gsi.installed)
    $('#gsi-target').text(data.gsi.target)
    $('#gamemode').text(data.gamemode)
    $('#map').text(data.map)
    $('#recommendations').html('')
    if (data.recommendations) {
      data.recommendations.forEach(function (rec) {
        $('#recommendations').append($('<li>' + rec + '</li>'))
      })
    }
    if (data.settings && $('#settings').is(':hidden')) {
      $('#install-path').val(data.settings.csgo_cfg_path)
    }
    $('#inventory').html('')
    if (data.inventory) {
      data.inventory.forEach(function (eqp) {
        $('#inventory').append($('<li>' + eqp + '</li>'))
      })
    }

    // hide some stuff
    var settingsError = data.gsi.installed === 'ERROR'
    var showInstallConfig = data.gsi.installed !== data.gsi.target
    showInstallConfig = showInstallConfig && !settingsError
    var showFail = !data.up
    var inGame = data.up && data.in_game
    $('#gsi-install-config').toggle(showInstallConfig)
    $('#csgo-down').toggle(showFail)
    $('#launch-game').toggle(showFail)
    $('#fail-icons').toggle(showFail)
    $('#out-of-game').toggle(!inGame)
    $('#in-game').toggle(inGame)
    $('#game-status').toggle(inGame)
    // don't hide the settings if there is no error, just show them if there is
    if (settingsError) {
      $('#settings').show()
    }

    setTimeout(update, 100)
  }).fail(function () {
    $('#fail-icons').show()
    $('#server-down').show()
    $('#csgo-down').hide()
    $('#out-of-game').show()
    $('#in-game').hide()

    setTimeout(update, 5000)
  })
}

$(function () {
  $('#js-loading').hide()
  $('#settings-link').click(function (evt) {
    $('#settings').show()
    evt.preventDefault()
  })
  $('#settings form').submit(function (evt) {
    var formdata = $('#settings form').serializeArray()
    var data = {}
    formdata.forEach(function (obj) {
      data[obj.name] = obj.value
    })
    data = JSON.stringify(data)
    $.ajax({
      type: 'POST',
      url: '/update-prefs',
      data: data,
      dataType: 'json',
      contentType: 'application/json'
    })
    $('#settings').hide()
    evt.preventDefault()
  })
  update()
  setTimeout(update, 100)
})
