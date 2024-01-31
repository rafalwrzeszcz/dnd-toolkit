import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

import main 1.0

Window {
    title: qsTr("RPG Qt")
    visible: true
    height: 480
    width: 640
    color: "#e4af79"

    Audio {
        id: audio
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        spacing: 10

        Button {
            text: "▶"
            onClicked: audio.playAudio()
        }
    }
}
