import pyarrow as pa
import pyarrow.ipc as ipc
import polars as pl

# 1) Use PyArrow to open and read the stream
with open("test", "rb") as f:
    reader = ipc.open_stream(f)  # specifically for the streaming format
    table = reader.read_all()

# 2) Convert the PyArrow Table to Polars DataFrame
df = pl.DataFrame(table)
print(df)
