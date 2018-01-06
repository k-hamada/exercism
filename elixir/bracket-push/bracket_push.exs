defmodule BracketPush do
  @doc """
  Checks that all the brackets and braces in the string are matched correctly, and nested correctly
  """
  @spec check_brackets(String.t) :: boolean
  def check_brackets(str) do
    check_bracket_pair([], String.graphemes(str))
  end

  defp check_bracket_pair([hd_pair | tl_pair], [head | tail])
    when (hd_pair == "[" and head == "]")
    or   (hd_pair == "{" and head == "}")
    or   (hd_pair == "(" and head == ")")
  do
    check_bracket_pair(tl_pair, tail)
  end

  defp check_bracket_pair(pair, [head | tail])
    when (hd(pair) == "[" and head != "]")
    or   (hd(pair) == "{" and head != "}")
    or   (hd(pair) == "(" and head != ")")
  do
    check_bracket_pair([head | pair], tail)
  end

  defp check_bracket_pair([hd_pair | _], _)
    when (hd_pair == "]")
    or   (hd_pair == "}")
    or   (hd_pair == ")")
  do
    false
  end

  defp check_bracket_pair([hd_pair | tl_pair], str)
    when (hd_pair != "[")
    and  (hd_pair != "{")
    and  (hd_pair != "(")
  do
    check_bracket_pair(tl_pair, str)
  end

  defp check_bracket_pair([], [head | tail]) do
    check_bracket_pair([head], tail)
  end

  defp check_bracket_pair([], []) do
    true
  end

  defp check_bracket_pair(_, []) do
    false
  end
end
