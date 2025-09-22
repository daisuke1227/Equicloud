-- Create the main keyspace for equicloud
CREATE KEYSPACE IF NOT EXISTS equicloud
WITH REPLICATION = {
    'class': 'SimpleStrategy',
    'replication_factor': 3
} AND DURABLE_WRITES = true;