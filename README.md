# omaphone

omaphone is Omarchy Mobile's **Phone** app. It is laid out for a phone held
upright (480 logical pixels wide) and follows your Omarchy theme. It
behaves like AOSP's Dialer app; none of AOSP's code is used.

> **Emergency calls (911, 112) don't work on this phone yet. Keep another
> phone for emergencies.** Phone says so on its dialpad. If you dial an
> emergency number, it shows a full-screen message and never places the
> call.

Phone talks to the modem through **omatel-agent**, the background service
it shares with Messages, from the
[omatel](https://github.com/derekross/omatel) repository. The agent keeps
running when the app is closed: it rings for incoming calls, writes the call log and notifies you of missed calls.
Phone reads the same stores and address book as the agent, and uses
omatel's shared crates, pinned to an omatel release tag.

## What you get

- A large dialpad with smart dial: typing digits suggests contacts by the
  letters on the keys, or by their numbers. Long-press 0 for +. Characters
  after `,` in a number are sent as touch tones once the call is answered.
- Favorites (your starred contacts, plus the numbers you call most),
  Recents (the call log, with missed calls in red), and Contacts. All three
  are tabs.
- Contacts: search, a contact page, an editor (name, numbers with labels,
  emails, a note, favorite), importing and exporting `.vcf` files, and
  blocking. Import takes Google's and Apple's exports and merges
  duplicates (see below).
- The in-call screen: a call timer, mute, speaker, a keypad for touch
  tones, and hang up. The answer screen offers Answer, Decline and Reply
  with a message.
- `tel:` and `callto:` links open the dialpad with the number filled in. A
  **Contacts** launcher entry opens the Contacts tab.

Not yet: call audio routing (speaker and mute reach the phone's audio once
Omarchy Mobile's call audio work lands; the in-call screen says so), hold,
swap and conference calls, visual voicemail, contact photos (initials
only), and emergency calls.

## Install

On Omarchy Mobile, install the `omaphone` package; it brings in
`omatel-agent`. Start the agent once for your user:

```
systemctl --user enable --now omatel-agent
```

To build the package from a checkout, run `bin/install` on the phone, or
`bin/cross package` on an x86_64 laptop (see
[docs/development.md](docs/development.md)).

## Use

Open **Phone** from the app launcher, or:

```
omaphone                    # the dialpad
omaphone tel:+12025550100   # the dialpad with a number filled in (never dials by itself)
omaphone --contacts         # also --recents, --favorites, --keypad
omaphone --add NUMBER       # a new contact with this number
omaphone --import FILE.vcf
omaphone FILE.vcf           # asks first, as when you open a .vcf from Files or a browser
```

Phone is registered for `text/vcard`, so opening a downloaded `.vcf`
offers to import it.

Without the agent the app still shows its data, with a banner saying the
service isn't running.

## Import contacts from Google or Apple

Get a `.vcf` file, copy it to the phone, then tap **Import contacts…** on
the Contacts tab (or in the menu), or open the file from Files or your
browser's downloads.

- **Google Contacts**: on contacts.google.com, select the contacts (or
  none, for all), choose **Export**, pick **vCard (for iOS Contacts)**
  and **Export**.
- **iCloud**: on icloud.com/contacts, select the contacts (Ctrl+A for
  all), open the **⋯** menu and choose **Export vCard**.
- **Mac**: in Contacts, select the contacts and choose **File > Export >
  Export vCard**.

A card joins a contact already here when it has the same UID, or the same
name and a shared number; its numbers, emails and other details are
added, and nothing is lost. Phone then says how many contacts were added,
merged and skipped. Custom labels (`Boat`, `Studio`) and photos are kept.

## Contacts with khard, scripts and AI agents

Contacts are plain vCard files, one per contact, in
`~/.local/share/contacts/<book>/` (a vdir); Phone and Messages pick up
changes other tools make. To use [khard](https://github.com/lucc/khard)
on them, add to `~/.config/khard/khard.conf` (Phone never writes it):

```
[addressbooks]
[[default]]
path = ~/.local/share/contacts/default/
```

or, for every book (recent khard):

```
[addressbooks]
[[contacts]]
path = ~/.local/share/contacts/*
type = discover
```

Scripts and AI agents can read the `.vcf` files directly. To change one,
write a new file and rename it over the old one. If Phone has that
contact open, it says "This contact changed elsewhere" and shows the new
version instead of overwriting it.

## Where things are kept

| What | Where |
| --- | --- |
| Call log, block list | `~/.local/share/omatel/calls.db, blocked.db` |
| Contacts | `~/.local/share/contacts/<book>/*.vcf` |
| Settings | `~/.config/omatel/settings.toml` |
| Logs | `journalctl -t omaphone` (states only, never numbers or text) |

omaphone is released under the GNU General Public License, version 3 or
later. Its theme support comes from omatel's `omatel-theme`, adapted from
[gliff](https://github.com/omacom/gliff) (MIT); see NOTICE.
