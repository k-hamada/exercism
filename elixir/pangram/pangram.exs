defmodule Pangram do
  @letters for n <- ?a..?z, do: << n :: utf8 >>

  @doc """
  Determines if a word or sentence is a pangram.
  A pangram is a sentence using every letter of the alphabet at least once.

  Returns a boolean.

    ## Examples

      iex> Pangram.pangram?("the quick brown fox jumps over the lazy dog")
      true

  """

  @spec pangram?(String.t) :: boolean
  def pangram?(sentence) do
    Enum.empty?(@letters -- String.graphemes(String.downcase(sentence)))
  end
end
