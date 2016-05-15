# CommonCalendar
Displays a not-necessarily-serious calendar.

## Concept

I was recently interested in various calendars used around the world, and couldn't help notice that there don't appear to be any calendars with clear licenses.  This is clearly not important, and I suppose that it's questionable whether a license would ever be required, but it interested me, so I dove in to create something that's plausible and not primarily derived from an existing calendar.  Thus, I present _The Commons Calendar_, itself released [CC-BY-SA](http://creativecommons.org/licenses/by-sa/4.0/legalcode), by contrast to the code's [GNU AGPL v3](https://opensource.org/licenses/agpl-3.0).

_The Commons Calendar_ tries to start from first (if ill-advised) principles and celebrates Free Culture.

### Other Calendars

The first step was to get a rough idea of the layout, by seeing the possible breakdowns of the year with various small numbers of intercalary days.

 - 365's prime factors are 5 x 73, which turns out to be the [Discordian calendar](https://en.wikipedia.org/wiki/Discordian_calendar).
 
 - 364's prime factors (+1) are 2 x 2 x 7 x 13, which happens to be the [Positivist](https://en.wikipedia.org/wiki/Positivist_calendar)/[Georgian calendar](https://en.wikipedia.org/wiki/Hugh_Jones_%28professor%29#Georgian_Calendar_.28Pancronometer.29).

 - 363's prime factors (+2) are 3 x 11 x 11.

 - 362's prime factors (+3) are 2 x 181, not particularly useful.

 - 361's prime factors (+4) are 19 x 19, which is used in the [Baha'i calendar](https://en.wikipedia.org/wiki/Bah%C3%A1'%C3%AD_calendar).

 - 360's prime factors (+5) are 2 x 2 x 2 x 3 x 3 x 5, which would be a genericized twelve-month, thirty-day calendar, a simplified [Gregorian calendar](https://en.wikipedia.org/wiki/Gregorian_calendar).

 - 359 (+6) is prime, _definitely_ not useful.

 - 358's prime factors (+7) are 2 x 179, again, not very useful.

Beyond a week of intercalary days seems pointless, so process of elimination leaves _The Commons Calendar_ with a plausible, if quirky, eleven months of thirty-three days each, plus two intercalary days.

### The Week

A seven-day week _could_ work, but isn't necessary on an entirely new calendar.  Ideal cases would relate to the number of days in a month.  Discarding eleven-day weeks (though this idea has potential), doubling the other factor (three) to six is week-like, was apparently experimented with (along with five) by the Soviet Union, and makes all months six weeks long, nearly square.

The days need names, of course.  In many cultures, the days are named for gods, but that seems crass in 2016.  However, the same sort of inspirational role may be supplied by major Free projects.  After some thinking, the most prominent and diverse projects seemed to be [Arduino](http://www.arduino.cc/), _[Sita Sings the Blues](http://www.sitasingstheblues.com/)_, [Wikipedia](https://en.wikipedia.org/wiki/Main_Page) and [Wikimedia](https://wikimediafoundation.org/wiki/Home)'s related Free projects, the [Linux kernel](https://www.kernel.org/), [GNU](https://gnu.org/), and [Creative Commons](https://creativecommons.org/).

Accounting for convenient pronunciation and simplicity (and replacing the name of Linux with its mascot, [Tux](https://en.wikipedia.org/wiki/Tux)), that suggests a list like...

 - Duinday
 - Sitaday
 - Wikiday
 - Tuxday
 - Gnuday
 - Commonday

That's one software project, one hardware, two broad projects representing licenses, a movie, and an encyclopedia (and software type).  Sita even refers to the consort of a god, maintaining a link to traditional calendars.

I haven't worked out a full eleven-day week as an alternative, but it has occurred to me that a major potential objection to a six-day week is that it changes the weekday/weekend balance that most of the world is currently familiar with, whereas eleven days with a tradition of three off (say, the first, middle, and last days) comes out to _almost_ the same ratio of days.  As hinted previously, the eleven-day week would also divide evenly into each month, making for a perpetual calendar.

### Months of the Year

As mentioned, there are eleven months, which would clearly be inconvenient in the real world, but that's not really the point, here.

I decided that it made sense for the months to celebrate various principles of Free Culture.  Those that came to mind were freedom, transparency, participation, collaboration, sharing, replication, adaptation, equality, decentralization, attribution, and community.  However, just using words in some language seemed futile, so I turned to mythology...or, rather, mythologies that are part of Free Culture.

Where can we find Free Culture pantheons?  The three sources that came to mind were Dunsany's [The Gods of Pegāna](https://en.wikipedia.org/wiki/The_Gods_of_Peg%C4%81na) (in the public domain), the fake [Illyrian gods](https://en.wikipedia.org/w/index.php?title=Paleo-Balkanic_mythology&oldid=192411335) posted to Wikipedia ([CC-BY-SA](https://en.wikipedia.org/wiki/Wikipedia:Text_of_Creative_Commons_Attribution-ShareAlike_3.0_Unported_License), being first published on Wikipedia), [William Blake's mythology](https://en.wikipedia.org/wiki/William_Blake's_mythology) (also public domain), and Voltaire's [Dictionnaire philosophique](https://en.wikipedia.org/wiki/Dictionnaire_philosophique), mentioning several "lesser gods" (also public domain).  There are others, of course, but these seemed like they had the most potential, and with some _mild_ stretching of bailiwicks, produced...

 - Freedom - Yarinareth (speed)
 - Participation - Skarl (stamina)
 - Transparency - Trogool (forbidden knowledge)
 - Collaboration - Mikon (friendship)
 - Sharing - Broket (fortune)
 - Replication - Pertunda (fertility)
 - Equality - Shkumbe (love)
 - Adaptation - Kib (evolution)
 - Decentralization - Jabim (broken things)
 - Attribution - Zodrak (ambition)
 - Community - Habaniah (hearth)

Organizing them in hopes of making the months feel like a progression through the year came to Jabim, Zodrak, Trogool, Yanar, Shkumbe, Habniah, Skarl, Mikon, Pertunda, Kib, and Broket.

### Epagomenal Days

Remember that the calendar was designed to be eleven months of thirty-three days, with two days left over.  The remaining days are [Intercalation](https://en.wikipedia.org/wiki/Intercalation_%28timekeeping%29).  For convenience, they are epagomenal days, outside of any month.

The first epagomenal day comes between Yanar and Shkumbe.  Peer Day is intended to celebrate freedom and equality.

The second comes between Broket and Jabim.  The Torrent Feast celebrates sharing and decentralization, and is intended to combine with the New Year celebration.

The third, when one is needed (see the section on Leap Years), comes between Skarl and Mikon.  The Immersion Feast celebrates participation and collaboration.

#### Alternatives

Given that a general term for days used for intercalation are _intercalary days_, I briefly considered naming the first one after the [INTERCAL](https://en.wikipedia.org/wiki/INTERCAL) programming language and the second after [Kvikkalkul](http://workbench.cadenhead.org/book/homepage24/kvikkalkul/), a similarly infamous programming language...

### Leap Years

As most people know to some degree, a solar year is currently around 365.2421897 days.  While traditional calendars approximate this fractional part as one fourth (0.25) with many corrections, a significantly better (if less convenient) approximation is eight thirty-thirds (0.2424...).

Therefore, a Commons Leap Year occurs at intervals of four years seven times, followed by a span of five years.

Like all approximations, this regime does still drift.  Here, the calendar will slip by a full day slightly less than every 4,264 years.  This is likely to be inconsequential for most purposes, but nevertheless, the first leap year after years numbered as multiples of 4,264 are to be skipped.  Presumably, further correction will be required at some point, but that is left as an exercise to the reader.

### Start Date

The trickiest part of _The Commons Calendar_ was deciding on a date from which to anchor the dates, the first of Jabim, Year 0.  A zero-year is, of course, a necessity for easily handling dates _prior_ to the calendar.

Early candidates included the releases of works important to Free Culture, like the [GNU Manifesto](https://en.wikipedia.org/wiki/GNU_Manifesto) (March AD 1985), the [GNU Public License](https://en.wikipedia.org/wiki/GNU_General_Public_License) (25 February AD 1989), the [Open Content Project](https://en.wikipedia.org/wiki/Open_Content_Project) (c AD 1998), [Creative Commons](https://en.wikipedia.org/wiki/Creative_Commons) (December AD 2002), and others.  None of those felt right, however.  It was similarly difficult to authenticate a first Free Culture work, with many candidates offered with little evidence and rarely specific dates.

The eventual decision was to attempt to go back to the beginning of time-keeping, the first authenticated date that we can map to a date on the modern calendar.  The best candidate for such a date, as of this writing (Spring 2016) is the 14th of Simanu in ancient Sumer, a Lunar eclipse known to have occurred on [April 4, 2094 BC](http://www.historychannel.com.au/classroom/day-in-history/534/earliest-record-of-a-lunar-eclipse).  This, then, is _The Commons Calendar_'s "epoch."

Presumably, others will be discovered, but the calendar will remain fixed.

(Please note:  Use of _BC_ and _AD_, rather than the secular _BCE_ and _CE_, is for clarity, both avoiding the similarity of the latter pair of terms and the lack of clarity of "common era.")

### Holidays

We have, of course, already seen four intended holidays:  New Year's Day, Peer Day, the Immersion Feast (in Leap Years), and the Torrent Feast.

In addition, there are holidays that are both naturally relevant and (accounting for drifting ancient calendars and nearby historical incidents) so common and widespread that they're presumably required for any calendar:  The solstices and equinoctes.

 - Summer Solstice, 12 Trogool:  On the longest day of the year, many summer holidays revolve around power and purification, with fire- or water-based rituals.  There can often be some nationalist tendencies, in the modern day.  In the United States, Independence Day and Flag day flank the solstice, but those weeks are celebrated around the world (under various names) with bonfires, fireworks, waterfronts, and often dancing.  Here, this is related to the [Free Software Definition](https://www.gnu.org/philosophy/free-sw.html)'s freedom to copy and improve works.
 
 - Autumnal (Southward) Equinox, 04 Habinah:  Harvest festivals and feasts abound, though naturally, many are pushed back to the actual harvest season.  Most regions have one, with Thanksgiving in the United States probably being the latest in the year.  Chuseok, Pomona, Mehregan, and so forth are also strongly related.  Here, harvest broadly resembles the freedom to redistribute works.
 
 - Winter Solstice, 29 Mikon:  On the longest night of the year, reflection on death and conservation abounds, often with a final feast of the season.  It also tends to be associated with solar or fire birth mythology, presumably due to the day being short and (in the Northern hemisphere) the sun being at its smallest.  Christmas and its related traditions figure into the period, as does Dongzhi, Korochun, Sanghamitta Day, Hanukkah, Shalako, Shab-e Chelleh, Yule, and so forth.  Winter traditions map to the freedom to use works for any purpose.
 
 - Vernal (Northward) Equinox, 21 Broket:  Spring brings an assortment of holidays with birth and rebirth themes and implicit or explicit fertility rites.  Easter, Passover, many calendar new years, Mother's Day, Sham el-Nessim, and so forth.  Planting and fertility, lastly, map to the freedom to study works, including their sources.

Note that these are the interpretation of the Northern Hemisphere.  In much of the Southern Hemisphere, the seasons are clearly offset by two in the cycle.  Additionally, examples may vary from the actual date, given that various calendars originate historically on Lunar calendars.

To round this out, International Workers' Day is recommended (28 Jabim), as it effectively celebrates collective problem-solving and the nameless, toiling workers, which seems to reflect the general Free Culture ethos.


