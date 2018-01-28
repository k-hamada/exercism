package flatten

func Flatten(i interface{}) interface{} {
	list := i.([]interface{})
	idx := 0
	ilen := len(list)
	result := make([]interface{}, 0)
	for idx < ilen {
		if _, ok := list[idx].([]interface{}); ok {
			value := Flatten(list[idx]).([]interface{})
			j := 0
			jlen := len(value)
			for j < jlen {
				if value[j] != nil {
					result = append(result, value[j])
				}
				j += 1
			}
		} else {
			if list[idx] != nil {
				result = append(result, list[idx])
			}
		}
		idx += 1
	}
	return result
}
