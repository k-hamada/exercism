defmodule Anagram do
  @doc """
  Returns all candidates that are anagrams of, but not equal to, 'base'.
  """
  @spec match(String.t, [String.t]) :: [String.t]
  def match(base, candidates) do
    candidates
    |> Enum.reject(same_word(base))
    |> Enum.filter(match_word(base))
  end

  defp same_word(base) do
    fn (candidate) ->
      String.downcase(base) === String.downcase(candidate)
    end
  end

  defp match_word(base) do
    fn (candidate) ->
      letter_sort(base) === letter_sort(candidate)
    end
  end

  defp letter_sort(word) do
    word
    |> String.downcase
    |> String.graphemes
    |> Enum.sort
  end
end
