# Overview
Excel spreadsheets and their ability to automate calculations was one of those
things that started out as a simple utility and over time, just got more and
more unwieldy as more features got bolted onto a frame that was never meant to 
support them.

We now find ourselves in a situation where we essentially have to edit code 
in a tiny window, with code that is nigh-impossible to compartmentalize, share
and unit-test.

Spysheets should try to address this fundamental problem. The primary use of
spysheets are to automate tables and the calculations contained therein. It 
should also allow people to test the code that they write.

It is a table + a python IDE. It should get paired with a virtual environment,
unfortunately. We'll see if this actually takes off. Business customers will 
probably not appreciate having to learn Python, for example, and managing
cross-platform issues.

# Rules
- Every table is just a list of ``operations``.
- Each ``operation`` can consume any arbitrary block of cells. However, it must
  output a finite, rectangular block of cells.
- ``operations` can only add to tables, they cannot overwrite cells that already
  exist.