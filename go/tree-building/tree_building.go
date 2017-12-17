package tree

import (
	"errors"
	"sort"
)

type Record struct {
	ID, Parent int
}

type Node struct {
	ID       int
	Children []*Node
}

const rootID int = 0

func Build(records []Record) (*Node, error) {
	if len(records) == 0 {
		return nil, nil
	}

	sort.Slice(records, func(i, j int) bool {
		return records[i].ID < records[j].ID
	})

	tree := map[int]*Node{}
	for i, r := range records {
		if r.ID != i {
			return nil, errors.New("non continuous")
		}

		node := &Node{ID: r.ID}
		if r.ID == rootID {
			if r.Parent != 0 {
				return nil, errors.New("root node has parent")
			}
		} else {
			parent := tree[r.Parent]
			if parent == nil {
				return nil, errors.New("parent node not found")
			}
			parent.Children = append(parent.Children, node)
		}
		tree[r.ID] = node
	}
	return tree[rootID], nil
}
