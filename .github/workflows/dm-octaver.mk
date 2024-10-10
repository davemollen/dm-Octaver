######################################
#
# dm-octaver
#
######################################

DM_OCTAVER_VERSION = <SHA>
DM_OCTAVER_SITE = https://github.com/davemollen/dm-Octaver.git
DM_OCTAVER_SITE_METHOD = git
DM_OCTAVER_BUNDLES = dm-Octaver.lv2

define DM_OCTAVER_BUILD_CMDS
	rm -f $(@D)/lv2/dm-Octaver.lv2/libdm_octaver.so
	(cd $(@D)/lv2 && \
		~/.cargo/bin/cargo build $(MOD_PLUGIN_BUILDER_RUST_BUILD_FLAGS))
endef

define DM_OCTAVER_INSTALL_TARGET_CMDS
	$(INSTALL) -d $(TARGET_DIR)/usr/lib/lv2
	cp -rv $(@D)/lv2/dm-Octaver.lv2 $(TARGET_DIR)/usr/lib/lv2/
	$(INSTALL) -m 644 $(@D)/lv2/target/$(MOD_PLUGIN_BUILDER_RUST_TARGET)/release/libdm_octaver.so $(TARGET_DIR)/usr/lib/lv2/dm-Octaver.lv2/
endef

$(eval $(generic-package))
