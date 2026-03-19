using System;
using System.Collections.Generic;
using UnityEngine;

namespace WorldGenMcp.Runtime
{
    public static class WorldgenBoundaryContract
    {
        public const string Version = "1.0.0";

        public const string CreateWorld = "worldgen.create_world";
        public const string ConfigureWorld = "worldgen.configure_world";
        public const string ConfigureProfile = "worldgen.configure_profile";
        public const string ConfigureBiomes = "worldgen.configure_biomes";
        public const string ConfigureLandmarks = "worldgen.configure_landmarks";
        public const string RequestChunkBatch = "worldgen.request_chunk_batch";
        public const string RequestRegion = "worldgen.request_region";
        public const string SubmitMutationBatch = "worldgen.submit_mutation_batch";
        public const string ValidateWorld = "worldgen.validate_world";
        public const string SaveSnapshot = "worldgen.save_snapshot";
        public const string LoadSnapshot = "worldgen.load_snapshot";
        public const string GetWorldSummary = "worldgen.get_world_summary";
        public const string GetChunkInfo = "worldgen.get_chunk_info";
    }

    [Serializable]
    public class WorldgenApiError
    {
        public string code;
        public string message;
        public string details_json;
    }

    [Serializable]
    public class WorldgenResponseEnvelope<T>
    {
        public bool ok;
        public string tool;
        public string request_id;
        public string world_id;
        public T data;
        public List<string> warnings = new List<string>();
        public List<WorldgenApiError> errors = new List<WorldgenApiError>();
        public List<string> artifacts = new List<string>();
        public string version;
    }

    [Serializable]
    public class WorldgenWorldSeed
    {
        public string value;
        public string world_version;
        public bool has_salt;
        public string salt;
    }

    [Serializable]
    public class WorldgenChunkCoord
    {
        public int x;
        public int y;
        public int z;
        public bool has_z;
    }

    [Serializable]
    public class WorldgenChunkKey
    {
        public string world_id;
        public WorldgenChunkCoord coord;
        public string generation_version;
    }

    [Serializable]
    public class WorldgenRegionBounds
    {
        public WorldgenChunkCoord min;
        public WorldgenChunkCoord max;
    }

    [Serializable]
    public class WorldgenWorldProfile
    {
        public string name;
        public int world_size;
        public int chunk_size;
        public int sea_level;
        public int max_height;
        public float landmark_density;
        public float feature_density;
        public int streaming_radius;
        public string generation_version;
    }

    [Serializable]
    public class WorldgenBiomeDefinition
    {
        public string id;
        public string name;
        public Color terrain_color = Color.white;
        public Color fog_color = Color.white;
        public float noise_threshold;
        public int min_height;
        public int max_height;
    }

    [Serializable]
    public class WorldgenLandmarkDefinition
    {
        public string id;
        public string name;
        public float rarity;
        public int min_spacing;
    }

    [Serializable]
    public enum WorldgenMutationKind
    {
        TerrainEdit,
        PlaceObject,
        RemoveObject,
        TransformObject,
        SetState,
    }

    [Serializable]
    public class WorldgenMutation
    {
        public string mutation_id;
        public WorldgenMutationKind kind;
        public string target_id;
        public bool has_chunk;
        public WorldgenChunkCoord chunk;
        public string payload_json;
    }

    [Serializable]
    public class WorldgenChunkPayload
    {
        public WorldgenChunkKey chunk_key;
        public string biome_id;
        public List<string> feature_ids = new List<string>();
        public List<string> landmark_ids = new List<string>();
        public string mesh_reference;
        public string collision_reference;
        public string hash;
        public string version;
    }

    [Serializable]
    public class WorldgenSnapshotManifest
    {
        public string world_id;
        public string seed;
        public string generation_version;
        public string contract_version;
        public string profile_hash;
        public int chunk_count;
        public List<string> mutation_log = new List<string>();
        public List<string> exported_files = new List<string>();
    }

    [Serializable]
    public class WorldgenCreateWorldRequest
    {
        public string request_id;
        public WorldgenWorldSeed seed;
        public string world_name;
        public string profile_name;
        public int world_size;
        public int chunk_size;
        public bool enable_streaming;
        public bool assetless_mode;
        public string generation_version;
    }

    [Serializable]
    public class WorldgenConfigureWorldRequest
    {
        public string request_id;
        public string world_id;
        public bool has_world_size;
        public int world_size;
        public bool has_chunk_size;
        public int chunk_size;
        public bool has_sea_level;
        public int sea_level;
        public bool has_max_height;
        public int max_height;
        public bool has_streaming_radius;
        public int streaming_radius;
        public bool has_feature_density;
        public float feature_density;
        public bool has_landmark_density;
        public float landmark_density;
    }

    [Serializable]
    public class WorldgenRequestChunkBatchRequest
    {
        public string request_id;
        public string world_id;
        public List<WorldgenChunkCoord> chunks = new List<WorldgenChunkCoord>();
    }

    [Serializable]
    public class WorldgenRequestRegionRequest
    {
        public string request_id;
        public string world_id;
        public WorldgenRegionBounds region;
    }

    [Serializable]
    public class WorldgenSubmitMutationBatchRequest
    {
        public string request_id;
        public string world_id;
        public List<WorldgenMutation> mutations = new List<WorldgenMutation>();
    }

    [Serializable]
    public class WorldgenValidateWorldRequest
    {
        public string request_id;
        public string world_id;
        public bool strict;
        public bool has_region;
        public WorldgenRegionBounds region;
    }

    [Serializable]
    public class WorldgenSaveSnapshotRequest
    {
        public string request_id;
        public string world_id;
        public bool include_previews;
    }

    [Serializable]
    public class WorldgenLoadSnapshotRequest
    {
        public string request_id;
        public string snapshot_path;
    }

    [Serializable]
    public class WorldgenGetWorldSummaryRequest
    {
        public string request_id;
        public string world_id;
    }

    [Serializable]
    public class WorldgenGetChunkInfoRequest
    {
        public string request_id;
        public string world_id;
        public WorldgenChunkCoord chunk;
    }

    [Serializable]
    public class WorldgenCreateWorldData
    {
        public string world_id;
        public string profile_hash;
    }

    [Serializable]
    public class WorldgenWorldSummary
    {
        public string seed;
        public string profile_name;
        public int world_size;
        public int chunk_size;
        public int chunk_count;
        public int biome_count;
        public int landmark_count;
    }

    [Serializable]
    public class WorldgenValidationReport
    {
        public bool passed;
        public string metrics_json;
    }

    [Serializable]
    public class WorldgenChunkBatchData
    {
        public List<WorldgenChunkPayload> chunks = new List<WorldgenChunkPayload>();
    }

    [Serializable]
    public class WorldgenMutationBatchData
    {
        public List<string> applied = new List<string>();
        public List<string> rejected = new List<string>();
    }

    [Serializable]
    public class WorldgenSnapshotData
    {
        public WorldgenSnapshotManifest manifest;
    }
}
