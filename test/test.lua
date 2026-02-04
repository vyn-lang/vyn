local SIZE = 10

local inp_arr = {29, 12, 44, 9, 7, 97, 4, 33, 64, 35}
local res_arr = {}

for i = 1, SIZE do
    res_arr[i] = inp_arr[i]
end

for i = 1, SIZE - 1 do
    local swapped = false
    for j = 1, SIZE - i - 1 do
        if res_arr[j] > res_arr[j + 1] then
            local temp = res_arr[j]
            res_arr[j] = res_arr[j + 1]
            res_arr[j + 1] = temp
            swapped = true
        end
    end
    if not swapped then
        break
    end
end

print(table.concat(inp_arr, ", "))
print(table.concat(res_arr, ", "))
