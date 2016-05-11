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


