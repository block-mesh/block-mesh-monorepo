CREATE USER block_mesh_collector WITH PASSWORD 'securepassword';
CREATE DATABASE block_mesh_collector;
GRANT ALL PRIVILEGES ON DATABASE block_mesh_collector TO block_mesh_collector;
ALTER DATABASE block_mesh_collector OWNER TO block_mesh_collector;