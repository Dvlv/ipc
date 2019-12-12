import xmlrpc.client

proxy = xmlrpc.client.ServerProxy("http://127.0.0.1:8080")

point_x = input("X coord ")
point_y = input("Y coord ")

params = {"point": {"x": point_x, "y": point_y}, "m": 2}

print(f"moving({point_x},{point_y}) right by {params['m']}")
move_right = proxy.move_right(params)
print(move_right)
