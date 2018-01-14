defmodule Isogram do
  @doc """
  Determines if a word or sentence is an isogram
  """
  @spec isogram?(String.t) :: boolean
  def isogram?(sentence) do
    graphemes =
      sentence
      |> String.graphemes
      |> Enum.reject(&(&1 in ["-", " "]))
    count =
      graphemes
      |> Enum.count
    uniq_count =
      graphemes
      |> Enum.uniq
      |> Enum.count
    count == uniq_count
  end
end
