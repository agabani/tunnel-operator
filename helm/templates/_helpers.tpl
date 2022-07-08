{{/*
Expand the name of the chart.
*/}}
{{- define "tunnel-operator.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "tunnel-operator.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "tunnel-operator.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "tunnel-operator.labels" -}}
helm.sh/chart: {{ include "tunnel-operator.chart" . }}
{{ include "tunnel-operator.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "tunnel-operator.selectorLabels" -}}
app.kubernetes.io/name: {{ include "tunnel-operator.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the cluster role to use
*/}}
{{- define "tunnel-operator.clusterRoleName" -}}
{{- if .Values.clusterRole.create }}
{{- default (include "tunnel-operator.fullname" .) .Values.clusterRole.name }}
{{- else }}
{{- default "default" .Values.clusterRole.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the cluster role binding to use
*/}}
{{- define "tunnel-operator.clusterRoleBindingName" -}}
{{- if .Values.clusterRoleBinding.create }}
{{- default (include "tunnel-operator.fullname" .) .Values.clusterRoleBinding.name }}
{{- else }}
{{- default "default" .Values.clusterRoleBinding.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the namespace to use
*/}}
{{- define "tunnel-operator.namespaceName" -}}
{{- if .Values.namespace.create }}
{{- default (include "tunnel-operator.fullname" .) .Values.namespace.name }}
{{- else }}
{{- default "default" .Values.namespace.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the role to use
*/}}
{{- define "tunnel-operator.roleName" -}}
{{- if .Values.role.create }}
{{- default (include "tunnel-operator.fullname" .) .Values.role.name }}
{{- else }}
{{- default "default" .Values.role.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the role binding to use
*/}}
{{- define "tunnel-operator.roleBindingName" -}}
{{- if .Values.roleBinding.create }}
{{- default (include "tunnel-operator.fullname" .) .Values.roleBinding.name }}
{{- else }}
{{- default "default" .Values.roleBinding.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "tunnel-operator.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "tunnel-operator.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}
