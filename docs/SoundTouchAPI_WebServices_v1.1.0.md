Bose SoundTouch Webservices API

Bose Corporation

Version 1.1.0

1

CONTENTS

Contents

1 Document Version History

2 Acronyms and Deﬁnitions

3 Contact Info/Legal

4 Overview

CONTENTS

3

3

3

3

4

5

5

5

6

6

7

7

7

8

8

8

8

9

9

10

10

11

11

11

12

12

13

13

13

13

13

14

15

15

15

15

15

16

4.1 Special types used by the SoundTouch WSAPI

. . . . . . . . . . . . . . . . . . . . . . . . . .

5 General Status and Errors

6 API Methods/URLs

6.1

6.2

6.3

6.4

6.5

6.6

6.7

6.8

6.9

/key . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/select . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/sources . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/bassCapabilities . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/bass . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/getZone . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/setZone . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/addZoneSlave . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

/removeZoneSlave

. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

6.10 /now playing . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

6.11 /trackInfo . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

6.12 /volume . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

6.13 /presets . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

6.14 /info . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

6.15 /name . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7 WebSockets

7.1 WebSocket Asynchronous Notiﬁcations . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.1 PresetsChangedNotifyUI . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.2 RecentsUpdatedNotifyUI

. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.3 AcctModeChangedNotifyUI . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.4 ErrorNotiﬁcation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.5 NowPlayingChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.6 VolumeChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.7 BassChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.8 ZoneMapChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.9

SWUpdateStatusChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.10 SiteSurveyResultsChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.11 SourcesChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.12 NowSelectionChange . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.13 NetworkConnectionStatus . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

7.1.14 InfoChange, e.g., the device name changed . . . . . . . . . . . . . . . . . . . . . . . .

2

4 OVERVIEW

1 Document Version History

Version

Release Date

Description of Changes

1.0.0

December 5, 2014

• Initial Release

1.0.1

December 17, 2014

1.1.0

February 5, 2016

• Section 3 updated with a link to the License

Agreement

• Updated incorrect variable names to remove errant
“\” in sections: 6.10, 6.11, 6.14–6.17, 6.20, 7.5

• Corrected WebSockets port to 8080 (previous version

incorrectly listed 8090)

• Title/description corrections for section 7.2

• Minor Corrections to sections 6.3 and 6.4

• Fix various misspellings and typos.

• Added clarity around the /select command to
show how it can be used to select AUX and
Bluetooth sources where available

• Added instruction around initiating a WebSockets

connection with a speaker

• Rearranged the order of the documentation to

form more relevant groupings

2 Acronyms and Deﬁnitions

Acronyms

Expanded Term

Deﬁnition

API

REST

WS API

SSDP

MDNS

Application Programming Interface

A deﬁnition for how to interact with and use a software component

Representational State Transfer

A common type of web service API that is modeled around resources

Webservices API

An API made available by a web server

Simple Services Discovery Protocol

A discovery protocol that uses unicast and multicast over UDP

Multicast Domain Name System

A type of discovery protocol that requires zero conﬁguration

Bonjour

Apple’s implementation of MDNS

3 Contact Info/Legal

For any questions, comments, or suggestions for improvements please email us at SoundTouchAPI@bose.com

Use of this API material is subject to the API License Agreement, which can be found at developers.bose.com/SoundTouch-
API-License

4 Overview

These commands are the primary interface to command and control a Bose SoundTouch. They are sent
over HTTP on port 8090 to the SoundTouch device you would like to connect to using the GET and POST
methods.

3

4.1 Special types used by the SoundTouch WSAPI

4 OVERVIEW

4.1 Special types used by the SoundTouch WSAPI

ART_STATUS {

INVALID
SHOW_DEFAULT_IMAGE
DOWNLOADING
IMAGE_PRESENT

}

BOOL: "true" or "false"

INT: a 32-bit integer

IPADDR: an IP address, represented as a string

KEY_VALUE {
PLAY
PAUSE
STOP
PREV_TRACK
NEXT_TRACK
THUMBS_UP
THUMBS_DOWN
BOOKMARK
POWER
MUTE
VOLUME_UP
VOLUME_DOWN
PRESET_1
PRESET_2
PRESET_3
PRESET_4
PRESET_5
PRESET_6
AUX_INPUT
SHUFFLE_OFF
SHUFFLE_ON
REPEAT_OFF
REPEAT_ONE
REPEAT_ALL
PLAY_PAUSE
ADD_FAVORITE
REMOVE_FAVORITE
INVALID_KEY

}

KEY_STATE {

press
release

}

MACADDR: a MAC address, upcased, represented as a string

PLAY_STATUS {
PLAY_STATE
PAUSE_STATE
STOP_STATE
BUFFERING_STATE

4

6 API METHODS/URLS

INVALID_PLAY_STATUS

}

PRESET_ID: An integer, 1 through 6 inclusive

SOURCE_STATUS {
UNAVAILABLE
READY

}

STRING: any valid XML-escaped string

UINT: a 32-bit unsigned integer

UINT64: a 64-bit unsigned integer

URL: a URL, encoded as a string

Any get* command results in a HTTP GET command
Any set* command results in a HTTP POST command, i.e. requires a payload

5 General Status and Errors

For calls that do not have a special return payload, the default response is:

<status>$STRING</status>

For calls that can produce errors, the error response is:

<errors deviceID="$STRING">

<error value="$INT" name="$STRING" severity="$STRING">$STRING</error>
...
</errors>

For malformed requests, i.e., wrong value the response is:

<error>XML parse error (1:116): Error reading Attributes.</error>

<errors deviceID="D05FB8A9591D"><error value="1019" name="CLIENT_XML_ERROR"

severity="Unknown">1019</error></errors>

6 API Methods/URLs

6.1

/key

Description: Keys are used as a simple means to interact with the SoundTouch speaker. For a full listing of
supported keys please see the list under KEY VALUE in section 4.1

Send a remote button press to the device

GET:
N/A

5

6.2

/select

POST:

6 API METHODS/URLS

<key state="$KEY_STATE" sender="$KEY_SENDER">$KEY_VALUE</key>

In general, it is good practice to send 2 discrete HTTP POST calls, the ﬁrst using “press” as the key state,
and the second using “release” as the key state. Doing so simulates both the press and release action of
clicking a key. Possible values for “$KEY STATE” are “press” or “release”.

The back to back message bodies will look like the following:

<key state="press" sender="Gabbo">$KEY_VALUE</key>

<key state="release" sender="Gabbo">$KEY_VALUE</key>

6.2

/select

Description:

Use this /select API to select AUX or Bluetooth sources when available. Sources available via this /select

API will vary based on product. Use the /sources API to view the availability for the device.

GET:

N/A

POST:

Examples:

Sources available via this /select API will vary based on product.

Use the /sources API to view the availability for the device.

Below are some samples for Bluetooth and AUX

<ContentItem source="AUX" sourceAccount="AUX"></ContentItem>

<ContentItem source="AUX" sourceAccount="AUX3"></ContentItem>

<ContentItem source="BLUETOOTH"></ContentItem>

6.3

/sources

Description:

List all available content sources

GET:

<sources deviceID="$MACADDR">

<sourceItem source="$SOURCE" sourceAccount="$STRING"
status="$SOURCE_STATUS">$STRING</sourceItem>

...
</sources>

6

6.4

/bassCapabilities

6 API METHODS/URLS

POST:

N/A

6.4

/bassCapabilities

Description: Some speakers do not support the ability to customize the bass levels, use this to ﬁnd out
whether bass customization is supported

Get or set bassCapabilities

GET:

<bassCapabilities deviceID="$MACADDR">
<bassAvailable>$BOOL</bassAvailable>
<bassMin>$INT</bassMin>
<bassMax>$INT</bassMax>
<bassDefault>$INT</bassDefault>

</bassCapabilties>

POST:

N/A

6.5

/bass

Description: Sets or gets the current bass setting for a particular speaker. This may or may not be a
supported capability, use the /bassCapabilities to ﬁnd out whether a speaker supports bass conﬁguration

Get or set bass

GET:

<bass deviceID="$MACADDR">

<targetbass>$INT</targetbass>
<actualbass>$INT</actualbass>

</bass>

POST:

<bass>$INT</bass>

6.6

/getZone

Description:

Gets the current state of the multi-room zone from particular device

GET:

<zone master="$MACADDR">

<member ipaddress="$MASTER_IPADDR">"$MASTER_MACADDR"</member>
<member ipaddress="$SLAVE1_IPADDR">"$SLAVE1_MACADDR"</member>
...
</zone>

7

6 API METHODS/URLS

6.7

/setZone

6.7

/setZone

Description:

Creates a multi-room zone

GET:
N/A
POST:

<zone master="$MACADDR" senderIPAddress="$IPADDR">

<member ipaddress="$IPADDR">$MACADDR</member>
...
</zone>

6.8

/addZoneSlave

Description:

Add a slave to a “play everywhere” zone

GET:
N/A
POST:

<zone master="$MACADDR">

<member ipaddress="$IPADDR">$MACADDR</member>
...
</zone>

6.9

/removeZoneSlave

Description:

Take a slave out of a “play everywhere” zone

GET:
N/A
POST:

<zone master="$MACADDR">

<member ipaddress="$IPADDR">$MACADDR</member>
...
</zone>

6.10

/now playing

Description:

Gets all info about the currently playing media

GET:

8

6.11

/trackInfo

6 API METHODS/URLS

<nowPlaying deviceID="$MACADDR" source="$SOURCE">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="$STRING" isPresetable="$BOOL">

<itemName>$STRING</itemName>

</ContentItem>
<track>$STRING</track>
<artist>$STRING</artist>
<album>$STRING</album>
<stationName>$STRING</stationName>
<art artImageStatus="$ART_STATUS">$URL</art>
<playStatus>$PLAY_STATUS</playStatus>
<description>$STRING</description>
<stationLocation>$STRING</stationLocation>

</nowPlaying>

POST:
N/A

6.11

/trackInfo

Description:

Get track information

GET:

<nowPlaying deviceID="$MACADDR" source="$SOURCE">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="$STRING" isPresetable="$BOOL">
<itemName>$STRING</itemName>
</ContentItem>
<track>$STRING</track>
<artist>$STRING</artist>
<album>$STRING</album>
<stationName>$STRING</stationName>
<art artImageStatus="$ART_STATUS">$URL</art>
<playStatus>$PLAY_STATUS</playStatus>
<description>$STRING</description>
<stationLocation>$STRING</stationLocation>
</nowPlaying>

POST:
N/A

6.12

/volume

Description:

Get or Set the volume and mute status for this SoundTouch device

Volume ranges between 0, 100 inclusive

GET:

<volume deviceID="$MACADDR">

<targetvolume>$INT</targetvolume>
<actualvolume>$INT</actualvolume>
<muteenabled>$BOOL</muteenabled>

</volume>

9

6.13

/presets

6 API METHODS/URLS

POST:

<volume>$INT</volume>

6.13

/presets

Description: Presets are a core part of the SoundTouch ecosystem. A preset is used to set and recall a
speciﬁc music stream supported by the SoundTouch speaker

List of current Presets

GET:

<presets>

<preset id="$PRESET_ID" createdOn="$UINT64" updateOn="$UINT64">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="$STRING"

isPresetable="$BOOL">
<itemName>$STRING</itemName>

</ContentItem>

</preset>
...
</presets>

POST:
N/A

6.14

/info

Description:

Get device information; mostly static device info such as device id, type, IP address

(per component if applicable), cloud account ID, software version, product version

and component type and version

GET:

<info deviceID="$MACADDR">
<name>$STRING</name>
<type>$STRING</type>
<margeAccountUUID>$STRING</margeAccountUUID>
<components>

<component>

<componentCategory>$STRING</componentCategory>
<softwareVersion>$STRING</softwareVersion>
<serialNumber>$STRING</serialNumber>

</component>
...

</components>
<margeURL>$URL</margeURL>
<networkInfo type="$STRING">

<macAddress>$MACADDR</macAddress>
<ipAddress>$IPADDR</ipAddress>

</networkInfo>
...
</info>

10

7 WEBSOCKETS

6.15

/name

POST:

N/A

6.15

/name

Description:

Set the device name

GET:

N/A

POST:

<name>$STRING</name>

7 WebSockets

Notiﬁcations are server initiated WebSocket messages which inform client(s) of changes in SoundTouch
device. They serve to keep clients in sync with the server. They are sent over HTTP on port 8080 via a
WebSocket connection which is initiated from a WebSocket client. The WebSocket connection oﬀers an
advantage over HTTP because it allows for bidirectional communication, which allows for asynchronous
notiﬁcations to be initiated from the server side (SoundTouch device) to the client connection.

7.1 WebSocket Asynchronous Notiﬁcations

After a successful WebSocket connection has been established, the simplest thing a client can do is to listen
for the asynchronous notiﬁcations that are published by the SoundTouch device.

The incomplete example below shows examples of a single update notiﬁcation describing what changed on
the SoundTouch device. This will help inform the client, if it is interested, to perform a new request for the
updated values. In some cases the notiﬁcation does not contain the changed information, but for convenience,
in other cases it may.

Creating the websocket:

When creating a client websocket connection, be sure to specify the protocol as “gabbo”. An example
javascript example is shown below.

socket = new WebSocket("ws://$IP", "gabbo")

Examples:

<updates deviceID="$MACADDR">

</updates>

<updates deviceID="$MACADDR">

<volume>

<targetvolume>$INT</targetvolume>
<actualvolume>$INT</actualvolume>

</volume>

</updates>

11

7.1 WebSocket Asynchronous Notiﬁcations

7 WEBSOCKETS

7.1.1 PresetsChangedNotifyUI

Description: When a preset is changed in any way like added, cleared, or modiﬁed the SoundTouch speaker
will send this asynchronous notiﬁcation. This is a signal for the WS API client to request the new list of
presets via the /presets API

<updates deviceID="$MACADDR">

<presetsUpdated>
<presets>

<preset id="$INT">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount=""

isPresetable="$BOOL">

<itemName>$STRING</itemName>

</ContentItem>

</preset>
<preset id="$INT">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="$STRING"

isPresetable="$BOOL">

<itemName>STRING</itemName>

</ContentItem>

</preset>
<preset id="$INT">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="STRING"

isPresetable="$BOOL">

<itemName>$STRING</itemName>

</ContentItem>

</preset>
<preset id="$INT" createdOn="$UINT64" updatedOn="$UINT64">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="" isPresetable="$BOOL">

<itemName>$STRING</itemName>

</ContentItem>

</preset>

</presets>
</presetsUpdated>

</updates>

7.1.2 RecentsUpdatedNotifyUI

Description: When the recents list is changed in any way like a recent is added, removed, or moved within
the list, the SoundTouch speaker will send this asynchronous notiﬁcation. This is a signal for the WS API
client to request the new list of recents via the /recents API

<updates deviceID=’$MACADDR’>

<recentsUpdated>
<recents>

<recent deviceID="$MACADDR" utcTime="$UINT64">
<contentItem source="$SOURCE" location="$STRING" sourceAccount="$STRING"

isPresetable="$BOOL">

<itemName>$STRING</itemName>

</contentItem>
</recent>
<recent deviceID="$MACADDR" utcTime="$UINT64">
<contentItem source="$SOURCE" location="$STRING" sourceAccount="" isPresetable="$BOOL">

<itemName>$STRING</itemName>

</contentItem>
</recent>
<recent deviceID="$MACADDR" utcTime="$UINT64">

12

7.1 WebSocket Asynchronous Notiﬁcations

7 WEBSOCKETS

<contentItem source="$SOURCE" location="$STRING" sourceAccount="" isPresetable="$BOOL">

<itemName>$STRING</itemName>

</contentItem>
</recent>

</recents>
</recentsUpdated>

</updates>

7.1.3 AcctModeChangedNotifyUI

Description: When the SoundTouch speaker’s association with a cloud account changes then this asyn-
chronous notiﬁcation will be sent

<updates deviceID=’$MACADDR’>

<acctModeUpdated>
</acctModeUpdated>

</updates>"

7.1.4 ErrorNotiﬁcation

ErrorNotification

7.1.5 NowPlayingChange

<updates deviceID="$MACADDR">

<nowPlayingUpdated><nowPlaying deviceID="$MACADDR" source="$SOURCE">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="" isPresetable="$BOOL">

<itemName>$STRING</itemName>

</ContentItem>
<track/>
<artist/>
<album/>
<stationName>$STRING</stationName>
<art artImageStatus="$ART_STATUS">$URL</art>
<playStatus>$PLAY_STATUS</playStatus>
<description>$STRING</description>
<stationLocation>$STRING</stationLocation>
</nowPlaying>
</nowPlayingUpdated>

</updates>

7.1.6 VolumeChange

<updates deviceID="$MACADDR">

<volumeUpdated/>

</updates>

7.1.7 BassChange

13

7.1 WebSocket Asynchronous Notiﬁcations

7 WEBSOCKETS

<updates deviceID="$MACADDR">

<bassUpdated/>

</updates>

7.1.8 ZoneMapChange

<updates deviceID="$MACADDR">

<zoneUpdated/>

</updates>

* Slave device joining a zone

<updates deviceID="slave $MACADDR">

<zoneUpdated/>

</updates>
<updates deviceID="slave $MACADDR">

<volumeUpdated/>

</updates>
<updates deviceID="slave $MACADDR">

<volumeUpdated/>

</updates>
<updates deviceID="slave $MACADDR">

<nowPlayingUpdated/>

</updates>

* Slave device leaving a zone

<updates deviceID="slave $MACADDR">

<zoneUpdated/>

</updates>
<updates deviceID="slave $MACADDR">

<nowPlayingUpdated/>

</updates>

* Master device notiﬁes any time a slave device joins its zone

<updates deviceID="slave $MACADDR">

<zoneUpdated/>

</updates>
<updates deviceID="slave $MACADDR">

<nowPlayingUpdated/>

</updates>

* Master device notiﬁes any time a slave device leaves its zone

14

7.1 WebSocket Asynchronous Notiﬁcations

7 WEBSOCKETS

<updates deviceID="$MACADDR">

<zoneUpdated/>

</updates>
<updates deviceID="$MACADDR">

<zoneUpdated/>

</updates>

7.1.9 SWUpdateStatusChange

Description: While this may happen in general, it is not important and there is no need to take any action
when this is received

<updates deviceID="$MACADDR">
<swUpdateStatusUpdated/>

</updates>

7.1.10 SiteSurveyResultsChange

Description: While this may happen in general, it is not important and there is no need to take any action
when this is received

<updates deviceID="$MACADDR">
<siteSurveyResultsUpdated/>

</updates>

7.1.11 SourcesChange

<updates deviceID="$MACADDR">

<sourcesUpdated/>

</updates>

7.1.12 NowSelectionChange

<updates deviceID="$MACADDR">

<nowSelectionUpdated>
<preset id="$INT">

<ContentItem source="$SOURCE" location="$STRING" sourceAccount="$STRING"

isPresetable="$BOOL">

<itemName>$STRING</itemName>

</ContentItem>

</preset>

</nowSelectionUpdated>

</updates>

7.1.13 NetworkConnectionStatus

<updates deviceID="$MACADDR">
<connectionStateUpdated/>

</updates>

15

7.1 WebSocket Asynchronous Notiﬁcations

7 WEBSOCKETS

7.1.14 InfoChange, e.g., the device name changed

<updates deviceID="$MACADDR">

<infoUpdated/>

</updates>

16


