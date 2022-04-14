const sampleTOML = """
# configuration

[monday]


[monday.strategy]
mode = "Limit"
limit_mwh = 1

[monday.base]
mode = "AlwaysOn"

[tuesday]

[tuesday.base]
mode = "AlwaysOn"

[wednesday]

[thursday]

[friday]

[saturday]
hours_always_on = [1, 2, 3, 4]

[sunday]
""";