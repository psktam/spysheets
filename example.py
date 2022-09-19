TABLE = spysheet.Table(filepath)
# Or, from spysheets import SessionTable as TABLE


@(TABLE.link["A", 1:].to["A", TABLE.last_row])
def calculate_mean(input_selection):
    return np.mean(input_selection)

# You can also define anonymous calls, I guess:
anon = Table.link["A", 1:].to["A", TABLE.last_row](
    lambda input_selection: np.mean(input_selection)
)

# Use this to break loops and sequence calculations in the order that they 
# should be performed.

# This function could be moved over to the UI, I suppose.
TABLE.order([
    calculate_mean,
])

# All functions not listed in order will get called after the ones in order get
# called.

# Each table can only map to a single file called __table__.py