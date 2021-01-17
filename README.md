# stream-to-journald

A simple program that reads stdin and sends name-value pairs to the systemd journal. If an argument is provided that will be used as the field separator when parsing lines.

Designed to be used with apache's pipe log commands, e.g.
`CustomLog "| /usr/bin/stream-to-journald |" "MESSAGE=%v %h %r %s \"%r\" %>s %b|LOG=access_log|VHOST=%V|REQUEST=%r|STATUS=%s|CLIENT=%h|ERROR_ID=%L|USER_AGENT=%{User-Agent}i|SYSLOG_IDENTIFIER=apache2|PRIORITY=6"`
