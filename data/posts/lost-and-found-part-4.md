The "Lost and Found" series continues from [Part 3][] with a deeper look into
the practice of software development and the kinds of wins and losses I've seen
along the way. Rewinding back a bit to my days working in technical support, I
remember how dreadful it was trying to troubleshoot something that appeared to
be a bug in the software. Many times we had no way of knowing what the cause was
or where it stemmed from. The primary reason for this, as you might guess, was
that we had **very** poor error handling. On top of that, our code base had
little to no logging. If there was any error handling at all, it was done by
passing around an error message in [$@][]. For those unfamiliar with Perl or how
it handles errors, $@ is meant to store the error message from the last [eval()][]
operator; however, many Perl developers set it manually when an error occurs. I
can't quite understand how this was ever standard practice. The obvious issues
with this approach are that it gives very little detail about the error other
than a string, the message can be easily clobbered or overwritten and worst of
all there's no stack trace or information about where the error was originally
set. If you were lucky enough to get a valid error message, you'd have to [grep][]
the code base to find where the error got set, hoping that there were only a few
places to look.

With that bit of backstory out of the way, you can imagine how much of a win it
was to finally get some logging in place. One of the first things I helped with
when we started our software rewrite was to implement [Log::Log4perl][] [^1]. I
think this, above anything else we did, saved hundreds of developer hours. I can
recall numerous occasions where I was working in very complicated parts of the
system and discovered defects within seconds, simply by having detailed DEBUG
and ERROR logging noting the module, method, and line number. This was long
before we had given any serious consideration to unit tests, but despite the
manual effort, it was a huge step up from before. Of course, the one caveat with
this approach is that after months or years when you've hardened various
sections of your code, the logs become cluttered with useless messages. Things
get to the point where sifting through the log is like trying to decode [The
Matrix][]. This makes it difficult to find bugs in more recently written
code. After a long enough period of time, those DEBUG statements should be
removed or changed to TRACE statements instead.

So, logging is great, especially during the early stages of development when
lots of testing/debugging is going on during integration testing. What's better?
Unit tests. I've heard a lot of people talk about unit tests and many of them
have varied definitions of what they are and what they are not. The specific
definition is going to depend on personal preference and what is required of the
project you're working on. Regardless of the definition, however, some key
principles should apply; small in scope, done by the developer implementing the
code, and fast. Some people refer to unit tests as if they were integration
tests and use the terms interchangeably. I think this is a mistake, if you've
got integration tests set up ([Selenium][], for example), calling them unit tests
give you a false sense of confidence in your test coverage. It's a given that
having **some** tests is better than having no tests, but ensuring unit tests
are in place gives you finer control and helps ensure modularity, clean
interfaces, and encapsulation. This is especially important if you have an [MVC][]
or Model-View-Controller architecture.

Which, I believe, leads to one of our largest losses. The lack of unit tests
from the start of the project resulted in much of our business logic (Controller
in MVC) mixed up with our View code. This means that when we did get around to
writing unit tests, it was now much harder. In Python/Django that means ensuring
a valid request context. In Perl, it usually means having a CGI instance. It's
extra overhead to have to worry about because there's more mutable state that
needs to be set up to test a View instead of testing a method in a
Controller. We also have a lot of our Model logic littered around as well. This
means that for the majority of the code, it's almost impossible to test without
having a real, live database up and running. All of these issues ultimately lead
to mental molasses that result in developers **not** writing any unit tests at
all. This, I think, is a great argument for having a more [TDD][]-centric
workflow. If you attack a problem solution with testing at the forefront of your
mind, you're forced to design code in a way that's easily tested.

Not having unit tests obviously results in more developer time debugging, due to
not having an early notification if something breaks. It also results in code
that is buggy and error-prone. However, the biggest impact is ultimately on the
QA team and eventually the customer. One of the slowest areas of the software
development cycle is in manual testing. This comes in the form of regular
testing of new features, regression testing, smoke-testing, load testing, and
penetration testing. This can add up to hundreds of hours of testing for even
the smallest of features, especially depending on the number and variety of
inputs. It's no wonder, then, that unit tests would save much of this time if
said tests can provide a high level of confidence that they cover what needs to
be covered. Having unit tests automate regression testing alone would justify
the time and energy to implement and maintain them. Just think about the
thousands of bugs your average enterprise system can manifest over its
lifetime. If every bug had a test ensuring it never occurred again in
production, how much time would the QA folks have to focus on other things like
smoke-testing?

I doubt any sane developer would refute the usefulness of unit tests. The
usefulness is rarely in question. The obstacles that are typically brought up
are time and difficulty of implementation. I've already mentioned that
implementation becomes much easier if you focus on testability first. Time,
however, is simply a matter of perspective. We humans are notoriously bad at
estimating time. It's especially bad if that time is split into hundreds of
small increments over the course of six months or if that time isn't felt by the
person doing the estimation. With unit tests, time put in now saves magnitudes
of time put in later. This saved time isn't just for the developer. It's also
for the fine folks in QA, Systems, and ultimately for the Customer or Support
Technician. All of that adds up. 

Let's run through a small example. Say you've got a reporting system that you
wrote without unit tests which report the number of calls taken in a day. Now
say there's a bug in that reporting system that doesn't account for calls that
were transferred to another extension. The customer, at the end of their week,
pulls up the report and is surprised at the low call count. So maybe they spend
30 minutes adding up their calls by looking through their emails, or [CRM][]
system or whatever. Finally, they come to the conclusion that the report is
wrong. They then call support and wait on the phone for 15 minutes before
getting to speak to someone. The technician spends 30 minutes troubleshooting
the issue (assuming they didn't already know about the bug) before informing the
customer. The technician files a bug report. Now say this happens to 100 other
customers. Some time later a QA person spends 30 minutes reproducing the issue
and filing an engineering ticket to fix. The ticket takes time from the
engineering project manager to update the issue, assign it to a developer,
prioritize it, etc. Let's say 10 minutes in total[^2]. Finally, the developer
gets it, tracks down the code, fixes it and tests it: 1 hour. QA re-tests it
before it gets deployed: 30 minutes. These are all very low estimates
considering the amount of overhead some companies have in coordinating
resources.

Have you been keeping track? Total time spent by all parties: ~105
hours[^3]. The time it should have taken to write unit tests to ensure the
reporting system accounted for all types of call traffic: ~2 hours[^4]. It's a
no-brainer. In [Part 5][] I'll talk about career stagnation and some ideas for
preventing it.


<small>photo credit: [Crossroads: Success or Failure][] via [StockMonkeys][] [cc][]</small>

[^1]: A Perl implementation of [Log4j][].
[^2]: This is a very efficient project manager!
[^3]: (30 minutes customer troubleshooting + 15 minutes waiting for support + average 15 minutes on the phone) * 100 customers = 100 hours. Then tack on about ~5 hours of internal administrative overhead to find/fix/release the bug.
[^4]: Depending on the complexity of the system and how easily the bug was found in the source.

[Part 3]: /lost-and-found-part-3 "\"Lost and Found\" : Part 3 - I do not think it means what you think it means"
[$@]: http://perldoc.perl.org/perlvar.html#Error-Variables
[eval()]: http://perldoc.perl.org/functions/eval.html
[grep]: http://perldoc.perl.org/functions/grep.html
[Log::Log4perl]: http://search.cpan.org/~mschilli/Log-Log4perl-1.46/lib/Log/Log4perl.pm
[Log4j]: http://logging.apache.org/log4j/2.x/
[The Matrix]: https://www.youtube.com/watch?v=3vAnuBtyEYE
[Selenium]: http://www.seleniumhq.org/
[MVC]: http://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller
[TDD]: http://en.wikipedia.org/wiki/Test-driven_development
[CRM]: http://en.wikipedia.org/wiki/Customer_relationship_management
[Part 5]: /lost-and-found-part-5 "\"Lost and Found\" : Part 5 - Turbo-charge your career and avoid stagnation"

[Crossroads: Success or Failure]: https://www.flickr.com/photos/86530412@N02/8226451812/
[StockMonkeys]: http://www.stockmonkeys.com/
[cc]: http://creativecommons.org/licenses/by/2.0/
