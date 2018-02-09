defmodule Markdown do
  @rule_heading ~r/\A(?<header>\#{1,6})\s+(?<inner>.*)/
  @rule_list ~r/\A\s*[\*|-]\s.*/
  @rule_listitem ~r/\A(?<nest>\s*)\*\s(?<inner>.*)/
  @rule_strong ~r/(?<before>.*?)(?:__(?<strong>.+)__)(?<after>.*)/
  @rule_emphasis ~r/(?<before>.*?)(?:_(?<emphasis>.+)_)(?<after>.*)/
  @doc """
    Parses a given string with Markdown syntax and returns the associated HTML for that string.

    ## Examples

    iex> Markdown.parse("This is a paragraph")
    "<p>This is a paragraph</p>"

    iex> Markdown.parse("#Header!\n* __Bold Item__\n* _Italic Item_")
    "<h1>Header!</h1><ul><li><em>Bold Item</em></li><li><i>Italic Item</i></li></ul>"
  """
  @spec parse(String.t()) :: String.t()
  def parse(text) do
    text
    |> to_ast
    |> output
  end

  defp to_ast(text) do
    text
    |> String.split("\n")
    |> to_ast_blocks([])
  end

  defp to_ast_blocks([], acc) do
    acc
    |> Enum.reverse()
  end

  defp to_ast_blocks([head | tail], acc) do
    cond do
      node = is_header_line?(head) ->
        new_ast_block = to_ast_heading(head)
        to_ast_blocks(tail, [new_ast_block | acc])

      is_list_line?(head) ->
        {lines, tail} = Enum.split_while([head | tail], &is_list_line?/1)
        new_ast_block = to_ast_list(lines)
        to_ast_blocks(tail, [new_ast_block | acc])

      true ->
        new_ast_block = to_ast_paragraph(head)
        to_ast_blocks(tail, [new_ast_block | acc])
    end
  end

  defp is_header_line?(line) do
    line
    |> String.match?(@rule_heading)
  end

  defp is_list_line?(line) do
    line
    |> String.match?(@rule_list)
  end

  defp to_ast_heading(line) do
    node = Regex.named_captures(@rule_heading, line)
    {:heading, to_ast_inline(node["inner"]), depth: String.length(node["header"])}
  end

  defp to_ast_list(lines) do
    {:list, Enum.map(lines, &to_ast_listitem/1)}
  end

  defp to_ast_listitem(line) do
    node = Regex.named_captures(@rule_listitem, line)
    {:listitem, to_ast_inline(node["inner"]), depth: String.length(node["nest"]) |> rem(2)}
  end

  defp to_ast_paragraph(line) do
    {:paragraph, to_ast_inline(line)}
  end

  defp to_ast_inline("") do
    {:empty}
  end

  defp to_ast_inline(text) do
    cond do
      node = Regex.named_captures(@rule_strong, text) ->
        [
          to_ast_inline(node["before"]),
          to_ast_strong(node["strong"]),
          to_ast_inline(node["after"])
        ]

      node = Regex.named_captures(@rule_emphasis, text) ->
        [
          to_ast_inline(node["before"]),
          to_ast_emphasis(node["emphasis"]),
          to_ast_inline(node["after"])
        ]

      true ->
        {:text, text}
    end
  end

  defp to_ast_strong(text) do
    {:strong, to_ast_inline(text)}
  end

  defp to_ast_emphasis(text) do
    {:emphasis, to_ast_inline(text)}
  end

  defp output(ast) when is_list(ast) do
    ast
    |> Enum.map(&output/1)
    |> Enum.join()
  end

  defp output({:paragraph, childrens}) do
    "<p>#{output(childrens)}</p>"
  end

  defp output({:heading, text, depth: depth}) do
    "<h#{depth}>#{output(text)}</h#{depth}>"
  end

  defp output({:list, listitems}) do
    "<ul>#{output(listitems)}</ul>"
  end

  defp output({:listitem, text, depth: _}) do
    "<li>#{output(text)}</li>"
  end

  defp output({:strong, value}) do
    "<strong>#{output(value)}</strong>"
  end

  defp output({:emphasis, value}) do
    "<em>#{output(value)}</em>"
  end

  defp output({:text, value}) do
    value
  end

  defp output({:empty}) do
    ""
  end
end
