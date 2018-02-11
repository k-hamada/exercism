if !System.get_env("EXERCISM_TEST_EXAMPLES") do
  Code.load_file("gigasecond.exs", __DIR__)
end

ExUnit.start
ExUnit.configure exclude: :pending, trace: true

defmodule GigasecondTest do
  use ExUnit.Case

  test "from 4/25/2011" do
    assert Gigasecond.from({{2011, 4, 25}, {0, 0, 0}}) == {{2043, 1, 1}, {1, 46, 40}}
  end

  test "from 6/13/1977" do
    assert Gigasecond.from({{1977, 6, 13}, {0, 0, 0}}) == {{2009, 2, 19}, {1, 46, 40}}
  end

  test "from 7/19/1959" do
    assert Gigasecond.from({{1959, 7, 19}, {0, 0, 0}}) == {{1991, 3, 27}, {1, 46, 40}}
  end

  test "yourself" do
    # customize these values for yourself
    # your_birthday = {{year1, month1, day1}, {0, 0, 0}}
    my_birthday = {{1988, 8, 4}, {0, 0, 0}}
    assert Gigasecond.from(my_birthday) == {{2020, 4, 12}, {1, 46, 40}}
  end
end
