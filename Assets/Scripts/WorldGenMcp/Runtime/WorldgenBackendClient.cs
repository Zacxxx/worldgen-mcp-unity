using System;
using System.Threading;
using System.Threading.Tasks;
using UnityEngine;

namespace WorldGenMcp.Runtime
{
    public interface IWorldgenTransport
    {
        Task<string> SendAsync(string toolName, string requestJson, CancellationToken cancellationToken = default);
    }

    public sealed class WorldgenBackendClient
    {
        private readonly IWorldgenTransport _transport;

        public WorldgenBackendClient(IWorldgenTransport transport)
        {
            _transport = transport ?? throw new ArgumentNullException(nameof(transport));
        }

        public Task<WorldgenResponseEnvelope<WorldgenCreateWorldData>> CreateWorldAsync(
            WorldgenCreateWorldRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenCreateWorldRequest, WorldgenCreateWorldData>(
                WorldgenBoundaryContract.CreateWorld,
                request,
                cancellationToken);
        }

        public Task<WorldgenResponseEnvelope<WorldgenWorldSummary>> GetWorldSummaryAsync(
            WorldgenGetWorldSummaryRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenGetWorldSummaryRequest, WorldgenWorldSummary>(
                WorldgenBoundaryContract.GetWorldSummary,
                request,
                cancellationToken);
        }

        public Task<WorldgenResponseEnvelope<WorldgenValidationReport>> ValidateWorldAsync(
            WorldgenValidateWorldRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenValidateWorldRequest, WorldgenValidationReport>(
                WorldgenBoundaryContract.ValidateWorld,
                request,
                cancellationToken);
        }

        public Task<WorldgenResponseEnvelope<WorldgenChunkBatchData>> RequestChunkBatchAsync(
            WorldgenRequestChunkBatchRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenRequestChunkBatchRequest, WorldgenChunkBatchData>(
                WorldgenBoundaryContract.RequestChunkBatch,
                request,
                cancellationToken);
        }

        public Task<WorldgenResponseEnvelope<WorldgenMutationBatchData>> SubmitMutationBatchAsync(
            WorldgenSubmitMutationBatchRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenSubmitMutationBatchRequest, WorldgenMutationBatchData>(
                WorldgenBoundaryContract.SubmitMutationBatch,
                request,
                cancellationToken);
        }

        public Task<WorldgenResponseEnvelope<WorldgenSnapshotData>> SaveSnapshotAsync(
            WorldgenSaveSnapshotRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenSaveSnapshotRequest, WorldgenSnapshotData>(
                WorldgenBoundaryContract.SaveSnapshot,
                request,
                cancellationToken);
        }

        public Task<WorldgenResponseEnvelope<WorldgenSnapshotData>> LoadSnapshotAsync(
            WorldgenLoadSnapshotRequest request,
            CancellationToken cancellationToken = default)
        {
            return SendAsync<WorldgenLoadSnapshotRequest, WorldgenSnapshotData>(
                WorldgenBoundaryContract.LoadSnapshot,
                request,
                cancellationToken);
        }

        public async Task<WorldgenResponseEnvelope<TResponse>> SendAsync<TRequest, TResponse>(
            string toolName,
            TRequest request,
            CancellationToken cancellationToken = default)
        {
            string requestJson = JsonUtility.ToJson(request, false);
            string responseJson = await _transport.SendAsync(toolName, requestJson, cancellationToken);
            if (string.IsNullOrWhiteSpace(responseJson))
            {
                throw new InvalidOperationException($"Transport returned an empty response for {toolName}.");
            }

            WorldgenResponseEnvelope<TResponse> response = JsonUtility.FromJson<WorldgenResponseEnvelope<TResponse>>(responseJson);
            if (response == null)
            {
                throw new InvalidOperationException($"Unable to parse backend response for {toolName}.");
            }

            return response;
        }
    }
}
