#RUST_BACKTRACE=1 target/debug/rusty --database /tmp/coin7.sqlite3 jsonfeed
#RUST_BACKTRACE=1 target/debug/rusty --database /tmp/coin6.sqlite3 update

rm -f /tmp/rusty.sqlite3
diesel migration run
RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 list
RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 subscribe -d http://www.geekzone.fr/feed/podcast/dans-le-canap
RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 list
RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 pending
RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 subscribe -d http://www.geekzone.fr/feed/podcast/dans-le-canap
# RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 unsubscribe 1
# RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 list
# RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 subscribe https://feeds.soundcloud.com/users/soundcloud:users:139721529/sounds.rss
# RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 downloaded


 #RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 subscribe -d http://valnuit.lepodcast.fr/rss
 # RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 download-dir  
 # RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 download-dir ~/ 
 # RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 download-dir  
 # RUST_BACKTRACE=1 target/debug/rusty --database /tmp/rusty.sqlite3 download 23 
