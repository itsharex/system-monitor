<template>
  <div class="floating-monitor" data-tauri-drag-region :style="monitorStyles">
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <div class="loading-spinner"></div>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <div class="error-icon">⚠️</div>
    </div>

    <!-- 数据显示 -->
    <div v-else class="monitor-data">
      <span class="data-label">CPU</span>
      <span class="data-value">{{ getCpuUsage }}%</span>

      <span class="data-divider">|</span>

      <span class="data-label">内存</span>
      <span class="data-value">{{ getMemoryUsage }}%</span>

      <template v-if="settings.enableGpuMonitor">
        <span class="data-divider">|</span>
        <span class="data-label">GPU</span>
        <span v-if="gpuInfo" class="data-value">{{ Math.round(gpuInfo.usage_percent) }}%</span>
        <span v-else class="data-value">--</span>
      </template>

      <span v-if="settings.enableNetworkMonitor && systemInfo?.network" class="data-divider">|</span>

      <span v-if="settings.enableNetworkMonitor && systemInfo?.network" class="data-label">网络</span>
      <span v-if="settings.enableNetworkMonitor && systemInfo?.network" class="data-value network-values">
        <div class="network-download">↓{{ networkSpeed.download }}</div>
        <div class="network-upload">↑{{ networkSpeed.upload }}</div>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useSystemMonitor } from '@/composables/useSystemMonitor'
import { useSystemStore } from '@/stores/system'
import { useSettingsStore } from '@/stores/settings'

// 使用系统监控组合式函数
const {
  systemInfo,
  gpuInfo,
  error,
  networkSpeed,
} = useSystemMonitor(true, 1000)

// 使用系统store
const systemStore = useSystemStore()
const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)

// 计算属性
const loading = computed(() => {
  // 如果有错误信息，不显示加载状态
  if (error?.value) return false

  // 如果系统信息不存在，显示加载状态
  return !systemInfo?.value
})

// 获取CPU使用率
const getCpuUsage = computed(() => {
  if (!systemInfo?.value) return 0
  return Math.round(systemInfo.value.cpu_usage)
})

// 获取内存使用率
const getMemoryUsage = computed(() => {
  if (!systemInfo?.value?.memory) return 0
  return Math.round(systemInfo.value.memory.usage_percent)
})

const monitorStyles = computed(() => {
  const opacity = (settings.value.opacity ?? 100) / 100
  const color = settings.value.themeColor || '#3b82f6'
  return {
    opacity,
    borderColor: color,
    boxShadow: `0 8px 20px ${color}33`,
    '--monitor-accent': color
  }
})

// 组件挂载时测试GPU监控功能
onMounted(async () => {
  await settingsStore.ensureInitialized()
  // 测试GPU监控状态
  const [isAvailable, errorMessage] = await systemStore.getGpuMonitorStatus()
  console.log('🎮 GPU监控状态:', {
    可用: isAvailable,
    错误信息: errorMessage
  })

  if (isAvailable) {
    // 获取GPU名称
    const gpuNames = await systemStore.getGpuNames()
    console.log('🎮 检测到的GPU:', gpuNames)

    // 获取详细GPU信息
    const detailedInfo = await systemStore.getDetailedGpuInfo(0)
    console.log('🎮 详细GPU信息:\n', detailedInfo)
  } else {
    console.log('🎮 GPU监控不可用，将不显示GPU信息')
  }
})

</script>

<style scoped>
/* 极简悬浮窗设计 */
.floating-monitor {
  height: 39px;
  font-weight: bold;
  border-radius: 20px;
  line-height: 39px;
  padding: 0 16px;
  border: 1px solid transparent;
  transition: box-shadow 0.2s ease, border-color 0.2s ease, opacity 0.2s ease;
}

/* 加载状态 */
.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top: 2px solid white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

/* 错误状态 */
.error-state {
  display: flex;
  align-items: center;
  justify-content: center;
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

.error-icon {
  font-size: 24px;
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

/* 数据显示区域 */
.monitor-data {
  display: flex;
  align-items: center;
  gap: 8px;
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

.data-label {
  font-size: 15px;
  font-weight: bold;
  color: rgba(255, 255, 255, 0.95);
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.8);
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

.data-value {
  font-size: 14px;
  font-weight: 700;
  color: white;
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.9);
  min-width: 32px;
  letter-spacing: 0.5px;
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

.data-divider {
  color: var(--monitor-accent, rgba(255, 255, 255, 0.5));
  font-size: 14px;
  font-weight: 300;
  margin: 0 6px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.8);
  /* 允许拖动事件穿透 */
  pointer-events: none;
}

.network-values {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  line-height: 1.1;
}

.network-download,
.network-upload {
  font-size: 12px;
  letter-spacing: 0.3px;
}
</style>
