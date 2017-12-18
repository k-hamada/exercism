defmodule Words do
  @doc """
  Count the number of words in the sentence.

  Words are compared case-insensitively.
  """
  @spec count(String.t) :: map
  def count(sentence) do
    sentence
    |> String.downcase
    |> String.replace(~r/[!"#$%&'()*+,.\/:;<=>?@[\]^_`{|}]/, " ") # [:punct:] without '-'
    |> String.split
    |> Enum.group_by(&(&1))
    |> Enum.map(fn ({k, v}) -> {k, Enum.count(v)} end)
    |> Map.new
  end
end
